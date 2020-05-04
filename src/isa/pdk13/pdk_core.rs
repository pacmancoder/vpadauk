use super::{
    IoAddr, Byte, NearRamAddr, FarRamAddr, RomAddr, Word,
    ir::{IrSlot, IrOpcode},
    ops,
};

const IO_SPACE_SIZE: usize = 32;

const FLAGS_IO_ADDR: IoAddr = 0x00;
const SP_IO_ADDR: IoAddr = 0x02;
const INTEN_IO_ADDR: IoAddr = 0x04;

pub const FLAG_ZERO_MASK: Byte = 0x01;
pub const FLAG_ZERO_OFFSET: Byte = 0x00;
pub const FLAG_CARRY_MASK: Byte = 0x02;
pub const FLAG_CARRY_OFFSET: Byte = 0x01;
pub const FLAG_AUX_CARRY_MASK: Byte = 0x04;
pub const FLAG_AUX_CARRY_OFFSET: Byte = 0x02;
pub const FLAG_OVERFLOW_MASK: Byte = 0x08;
pub const FLAG_OVERFLOW_OFFSET: Byte = 0x03;
pub const ARITH_FLAGS_MASK: Byte =
    FLAG_ZERO_MASK | FLAG_CARRY_MASK | FLAG_AUX_CARRY_MASK | FLAG_OVERFLOW_MASK;


trait Bus {
    fn write_acc(&mut self, addr: IoAddr, value: Byte);
    fn read_acc(&self, addr: IoAddr) -> Byte;

    fn write_io(&mut self, addr: IoAddr, value: Byte);
    fn read_io(&self, addr: IoAddr) -> Byte;

    fn write_ram(&mut self, addr: NearRamAddr, value: Byte) {
        self.write_ram_far(addr as FarRamAddr, value);
    }

    fn read_ram(&self, addr: NearRamAddr) -> Byte {
        self.read_ram_far(addr as FarRamAddr)
    }

    fn write_ram_far(&mut self, addr: FarRamAddr, value: Byte);
    fn read_ram_far(&self, addr: FarRamAddr) -> Byte;

    fn read_rom(&self, addr: RomAddr) -> Word;
    fn read_ir(&self, addr: RomAddr) -> IrSlot;

    fn write_tim16(&mut self, value: Word);
    fn read_tim16(&self) -> Word;

    fn reset(&mut self);
    fn stop_exe(&mut self);
    fn stop_sys(&mut self);
    fn wdt_reset(&mut self);
}

enum PdkCoreState {
    Execute,
    Skip,
}

struct PdkCore<B: Bus> {
    acc: Byte,
    pc: RomAddr,
    state: PdkCoreState,
    global_interrupts: bool,
    bus: B,
}

impl<B: Bus> PdkCore<B> {
    pub fn new(bus: B) -> Self {
        Self {
            acc: 0,
            pc: 0,
            state: PdkCoreState::Execute,
            global_interrupts: false,
            bus
        }
    }

    pub fn bus(&mut self) -> &mut B {
        &mut self.bus
    }

    pub fn step(&mut self) {
        self.state = match self.state {
            PdkCoreState::Execute => self.execute(),
            PdkCoreState::Skip => PdkCoreState::Execute,
        }
    }

    fn execute(&mut self) -> PdkCoreState {
        let ir = self.bus.read_ir(self.pc);
        let old_flags = self.bus().read_io(FLAGS_IO_ADDR);
        let mut next_state = PdkCoreState::Execute;
        let mut pc_increment = 1;

        match ir.opcode() {
            IrOpcode::Ldsptl => {
                // LDSPTL
                // A <- LowByte@CodeMem(WORD[SP])
                // Read addr pointed by SP as little endian
                let sp = self.bus.read_io(SP_IO_ADDR);
                let addr = self.bus.read_ram(sp) as u16 |
                    ((self.bus.read_ram(sp.wrapping_add(1)) as u16) << 8);
                self.acc = self.bus.read_rom(addr) as u8
            },
            IrOpcode::Ldspth => {
                // LDSPTH
                // A <- HighByteB@Codemem(WORD[SP])
                // Read addr pointed by SP as little endian
                let sp = self.bus.read_io(SP_IO_ADDR);
                let addr = self.bus.read_ram(sp) as u16 |
                    ((self.bus.read_ram(sp.wrapping_add(1)) as u16) << 8);
                self.acc = (self.bus.read_rom(addr) >> 8) as u8
            },
            IrOpcode::Addca => {
                // SUBC A
                // A <- A + CF
                let (acc, flags) = ops::sub(
                    self.acc,
                    (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET,
                    old_flags);
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Subca => {
                // SUBC A
                // A <- A - CF
                let (acc, flags) = ops::sub(
                    self.acc,
                    (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET,
                    old_flags);
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Izsna => {
                // IZSN A
                let (acc, flags) = ops::add(self.acc, 1, old_flags);
                if flags & FLAG_ZERO_MASK != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Dzsna => {
                // DZSN A
                let (acc, flags) = ops::sub(self.acc, 1, old_flags);
                if flags & FLAG_ZERO_MASK != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Pcadda => {
                // PCADD A
                self.pc.wrapping_add(self.acc as u16);
                pc_increment = 0;
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Nota => {
                // NOT A
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                self.acc = !self.acc;
                if self.acc == 0 {
                    flags |= FLAG_ZERO_MASK;
                }
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Nega => {
                // NEG A
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                self.acc = (!self.acc).wrapping_add(1);
                if self.acc == 0 {
                    flags |= FLAG_ZERO_MASK;
                }
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Sra => {
                // SR A
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= (self.acc & 0x01) << FLAG_CARRY_OFFSET;
                self.acc >>= 1;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Sla => {
                // SL A
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= ((self.acc & 0x80) >> 7) << FLAG_CARRY_OFFSET;
                self.acc <<= 1;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Srca => {
                // SRC A
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= (self.acc & 0x01) << FLAG_CARRY_OFFSET;
                self.acc >>= 1;
                self.acc |= ((old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET) << 7;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Slca => {
                // SLC A
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= ((self.acc & 0x80) >> 7) << FLAG_CARRY_OFFSET;
                self.acc <<= 1;
                self.acc |= (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Swapa => {
                // SWAP A
                self.acc = ((self.acc & 0xF0) >> 4) | ((self.acc & 0x0F) << 4);
            },
            IrOpcode::Wdreset => {
                // WDTRESET
                self.bus.wdt_reset();
            },
            IrOpcode::Pushaf => {
                // PUSHAF
                let sp = self.bus.read_io(SP_IO_ADDR);
                self.bus.write_ram(sp, self.acc);
                self.bus.write_ram(sp.wrapping_add(1), self.bus.read_io(FLAGS_IO_ADDR));
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_add(2));
            },
            IrOpcode::Popaf => {
                // POPAF
                let sp = self.bus.read_io(SP_IO_ADDR);
                self.bus.write_io(FLAGS_IO_ADDR, self.bus.read_ram(sp.wrapping_sub(1)));
                self.acc = self.bus.read_ram(sp.wrapping_sub(2));
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
            },
            IrOpcode::Reset => {
                // RESET
                self.bus.reset();
                self.pc = 0;
                self.acc = 0;
                self.global_interrupts = false;
                pc_increment = 0;
            },
            IrOpcode::Stopsys => {
                // STOPSYS
                self.bus.stop_sys();
            },
            IrOpcode::Stopexe => {
                // STOPEXE
                self.bus.stop_exe()
            },
            IrOpcode::Engint => {
                // ENGINT
                self.global_interrupts = true;
            },
            IrOpcode::Disgint => {
                // DISGINT
                self.global_interrupts = false
            },
            IrOpcode::Ret => {
                // RET
                let sp = self.bus.read_io(SP_IO_ADDR);
                let pc = ((self.bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
                    | (self.bus.read_ram(sp.wrapping_sub(2)) as u16);
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
                self.pc = pc;
                pc_increment = 0;
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Reti => {
                // RETI
                let sp = self.bus.read_io(SP_IO_ADDR);
                let pc = ((self.bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
                    | (self.bus.read_ram(sp.wrapping_sub(2)) as u16);
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
                self.pc = pc;
                pc_increment = 0;
                self.global_interrupts = true;
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Mul => {
                // MUL
                // No implementation for mul yet
            },
            IrOpcode::Xorioa => {
                // XOR IO, A
                self.bus.write_io(ir.operand8(), self.bus.read_io(ir.operand8()) ^ self.acc);
            },
            IrOpcode::Movioa => {
                // MOV IO, A
                self.bus.write_io(ir.operand8(), self.acc);
            },
            IrOpcode::Movaio => {
                // MOV A, IO
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                self.acc = self.bus.read_io(ir.operand8());
                if self.acc == 0 {
                    flags |= FLAG_ZERO_MASK;
                }
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Stt16 => {
                // STT16 M
                let word = (self.bus.read_ram(ir.operand8()) as u16)
                    | ((self.bus.read_ram(ir.operand8()) as u16) << 8);
                self.bus.write_tim16(word);
            },
            IrOpcode::Ldt16 => {
                // LDT16 M
                let word = self.bus.read_tim16();
                self.bus.write_ram(ir.operand8(), word as u8);
                self.bus.write_ram(ir.operand8().wrapping_add(1), (word >> 8) as u8);
            },
            IrOpcode::Idxmma => {
                // IDXM M, A
                let addr = (self.bus.read_ram(ir.operand8()) as u16)
                    | ((self.bus.read_ram(ir.operand8().wrapping_add(1)) as u16) << 8);
                self.bus.write_ram_far(addr, self.acc);
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Idxmam => {
                // IDXM A, M
                let addr = (self.bus.read_ram(ir.operand8()) as u16)
                    | ((self.bus.read_ram(ir.operand8().wrapping_add(1)) as u16) << 8);
                self.acc = self.bus.read_ram_far(addr);
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Retk => {
                // RET k
                self.acc = ir.operand16() as u8;
                let sp = self.bus.read_io(SP_IO_ADDR);
                let pc = ((self.bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
                    | (self.bus.read_ram(sp.wrapping_sub(2)) as u16);
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
                self.pc = pc;
                pc_increment = 0;
            },
            IrOpcode::T0snm => {
                // T0SN M.n
                if self.bus.read_ram(ir.operand8()) & (1 << ir.operand16() as u8) == 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::T1snm => {
                // T1SN M.n
                if self.bus.read_ram(ir.operand8()) & (1 << ir.operand16() as u8) != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::Set0m => {
                // SET0 M.n
                self.bus.write_ram(
                    ir.operand8(),
                    self.bus.read_ram(ir.operand8()) | (1 << ir.operand16() as u8));
            },
            IrOpcode::Set1m => {
                // SET1 M.n
                self.bus.write_ram(
                    ir.operand8(),
                    self.bus.read_ram(ir.operand8()) & (!(1 << ir.operand16() as u8)));
            },
            IrOpcode::Addma => {
                // ADD M, A
                let (r, f) = ops::add(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Subma => {
                // SUB M, A
                let (r, f) = ops::sub(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Addcma => {
                // ADDC M, A
                let (r, f) = ops::addc(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Subcma => {
                // SUBC M, A
                let (r, f) = ops::subc(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Andma => {
                // AND M, A
                let (r, f) = ops::and(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Orma => {
                // OR M, A
                let (r, f) = ops::or(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Xorma => {
                // XOR M, A
                let (r, f) = ops::xor(self.bus.read_ram(ir.operand8()), self.acc, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Movma => {
                // MOV M, A
                self.bus.write_io(ir.operand8(), self.acc);
            },
            IrOpcode::Addam => {
                // ADD A, M
                let (r, f) = ops::add(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Subam => {
                // SUB A, M
                let (r, f) = ops::sub(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Addcam => {
                // ADDC A, M
                let (r, f) = ops::addc(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Subcam => {
                // SUBC A, M
                let (r, f) = ops::subc(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Andam => {
                // AND A, M
                let (r, f) = ops::and(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Oram => {
                // OR A, M
                let (r, f) = ops::or(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Xoram => {
                // XOR A, M
                let (r, f) = ops::xor(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Movam => {
                // MOV A, M
                self.acc = self.bus.read_ram(ir.operand8());
            },
            IrOpcode::Addcm => {
                // ADDC M
                let (r, f) = ops::addc(self.bus.read_ram(ir.operand8()), 0, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Subcm => {
                // SUBC M
                let (r, f) = ops::subc(self.bus.read_ram(ir.operand8()), 0, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Izsnm => {
                // IZSN M
                let (acc, flags) = ops::add(self.bus.read_ram(ir.operand8()), 1, old_flags);
                if flags & FLAG_ZERO_MASK != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
                self.bus.write_ram(ir.operand8(), acc);
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Dzsnm => {
                // DZSN M
                let (acc, flags) = ops::sub(self.bus.read_ram(ir.operand8()), 1, old_flags);
                if flags & FLAG_ZERO_MASK != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
                self.bus.write_ram(ir.operand8(), acc);
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Incm => {
                // INC M
                let (r, f) = ops::add(self.bus.read_ram(ir.operand8()), 1, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Decm => {
                // DEC M
                let (r, f) = ops::sub(self.bus.read_ram(ir.operand8()), 1, old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Clearm => {
                // CLEAR M
                self.bus.write_ram(ir.operand8(), 0);
            },
            IrOpcode::Xchm => {
                // XCH M
                let tmp = self.acc;
                self.acc = self.bus.read_ram(ir.operand8());
                self.bus.write_ram(ir.operand8(), tmp);
            },
            IrOpcode::Notm => {
                // NOT M
                let (r, f) = ops::not(self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Negm => {
                // NEG M
                let (r, f) = ops::neg(self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Srm => {
                // SR M
                let (r, f) = ops::sr(self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Slm => {
                // SL M
                let (r, f) = ops::sl(self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Srcm => {
                // SRC M
                let (r, f) = ops::src(self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Slcm => {
                // SLC M
                let (r, f) = ops::slc(self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_ram(ir.operand8(), self.acc);
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Ceqsnam => {
                // CEQSN A, M
                let (r, f) = ops::sub(self.acc, self.bus.read_ram(ir.operand8()), old_flags);
                self.bus.write_io(FLAGS_IO_ADDR, f);
                if (f & FLAG_ZERO_MASK != 0) {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::T0snio => {
                // T0SN IO.n
                if self.bus.read_io(ir.operand8()) & (1 << ir.operand16() as u8) == 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::T1snio => {
                // T1SN IO.n
                if self.bus.read_io(ir.operand8()) & (1 << ir.operand16() as u8) != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::Set0io => {
                // SET0 IO.n
                self.bus.write_io(
                    ir.operand8(),
                    self.bus.read_io(ir.operand8()) | (1 << ir.operand16() as u8));
            },
            IrOpcode::Set1io => {
                // SET1 IO.n
                self.bus.write_io(
                    ir.operand8(),
                    self.bus.read_io(ir.operand8()) & (!(1 << ir.operand16() as u8)));
            },
            IrOpcode::Addak => {
                // ADD A, k
                let (r, f) = ops::add(self.acc, ir.operand16() as u8, old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Subak => {
                // SUB A, k
                let (r, f) = ops::sub(self.acc, ir.operand16() as u8, old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Ceqsnak => {
                // CEQSN A, k
                let (r, f) = ops::sub(self.acc, ir.operand16() as u8, old_flags);
                self.bus.write_io(FLAGS_IO_ADDR, f);
                if (f & FLAG_ZERO_MASK != 0) {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::Andak => {
                // AND A, k
                let (r, f) = ops::and(self.acc, ir.operand16() as u8, old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Orak => {
                // OR A, k
                let (r, f) = ops::or(self.acc, ir.operand16() as u8, old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Xorak => {
                // XOR A, k
                let (r, f) = ops::xor(self.acc, ir.operand16() as u8, old_flags);
                self.acc = r;
                self.bus.write_io(FLAGS_IO_ADDR, f);
            },
            IrOpcode::Movak => {
                // MOV A, k
                self.acc = ir.operand16() as u8;
            },
            IrOpcode::Goto => {
                // GOTO k
                self.pc = ir.operand16();
                pc_increment = 0;
                next_state = PdkCoreState::Skip;

            },
            IrOpcode::Call => {
                // CALL k
                let next_pc = self.pc.wrapping_add(1);
                self.bus.write_ram(
                    self.bus.read_io(SP_IO_ADDR),
                    next_pc as u8);
                self.bus.write_ram(
                    self.bus.read_io(SP_IO_ADDR).wrapping_add(1),
                    (next_pc >> 8) as u8);
                self.pc = ir.operand16();
                self.bus.write_io(SP_IO_ADDR, self.bus.read_io(SP_IO_ADDR).wrapping_add(2));
                pc_increment = 0;
                next_state = PdkCoreState::Skip;
            },
            _ => {},
        }
        self.pc.wrapping_add(pc_increment);
        next_state
    }
}