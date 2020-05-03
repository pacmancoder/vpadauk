use super::{
    IoAddr, Byte, NearRamAddr, FarRamAddr, RomAddr, Word,
    ir::{IrSlot, IrOpcode},
};

const IO_SPACE_SIZE: usize = 32;

const FLAGS_IO_ADDR: IoAddr = 0x00;
const SP_IO_ADDR: IoAddr = 0x02;
const INTEN_IO_ADDR: IoAddr = 0x04;

const FLAG_ZERO_MASK: Byte = 0x01;
const FLAG_ZERO_OFFSET: Byte = 0x00;
const FLAG_CARRY_MASK: Byte = 0x02;
const FLAG_CARRY_OFFSET: Byte = 0x01;
const FLAG_AUX_CARRY_MASK: Byte = 0x04;
const FLAG_AUX_CARRY_OFFSET: Byte = 0x02;
const FLAG_OVERFLOW_MASK: Byte = 0x08;
const FLAG_OVERFLOW_OFFSET: Byte = 0x03;
const ARITH_FLAGS_MASK: Byte =
    FLAG_ZERO_MASK | FLAG_CARRY_MASK | FLAG_AUX_CARRY_MASK | FLAG_OVERFLOW_MASK;

// Overflow and aux carry flags calculation tables (from z80 emulators fuse/rustzx)
// https://github.com/pacmancoder/rustzx/blob/master/src/z80/tables/mod.rs

#[cfg_attr(rustfmt, rustfmt_skip)]
const AUX_CARRY_ADD_TABLE: [u8; 8] = [
    0, FLAG_AUX_CARRY_MASK, FLAG_AUX_CARRY_MASK, FLAG_AUX_CARRY_MASK,0, 0, 0, FLAG_AUX_CARRY_MASK
];
#[cfg_attr(rustfmt, rustfmt_skip)]
const AUX_CARRY_SUB_TABLE: [u8; 8] = [
    0, 0, FLAG_AUX_CARRY_MASK, 0, FLAG_AUX_CARRY_MASK, 0, FLAG_AUX_CARRY_MASK, FLAG_AUX_CARRY_MASK
];

const OVERFLOW_ADD_TABLE: [u8; 8] = [0, 0, 0, FLAG_OVERFLOW_MASK, FLAG_OVERFLOW_MASK, 0, 0, 0];
const OVERFLOW_SUB_TABLE: [u8; 8] = [0, FLAG_OVERFLOW_MASK, 0, 0, 0, 0, FLAG_OVERFLOW_MASK, 0];

fn make_flags_lookup_index(a: u8, b: u8, result: u8) -> usize {
    return (((a & 0x88) >> 3) | ((b & 0x88) >> 2) | ((result & 0x88) >> 1)) as usize;
}

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

    #[inline(always)]
    fn add(acc: Byte, value: Byte, old_flags: Byte) -> (Byte, Byte) {
        let mut flags = old_flags & !ARITH_FLAGS_MASK;
        let result = (acc as u16).wrapping_add(value as u16);
        let result8 = result as u8;
        let flags_lookup_index = make_flags_lookup_index(acc, value, result8);
        if result > 0xFF {
            flags |= FLAG_CARRY_MASK;
        }
        if result8 == 0 {
            flags |= FLAG_ZERO_MASK;
        }
        flags |= OVERFLOW_ADD_TABLE[flags_lookup_index];
        flags |= AUX_CARRY_ADD_TABLE[flags_lookup_index];
        (result8, flags)
    }

    #[inline(always)]
    fn sub(acc: Byte, value: Byte, old_flags: Byte) -> (Byte, Byte) {
        let mut flags = old_flags & !ARITH_FLAGS_MASK;
        let result = (acc as u16).wrapping_sub(value as u16);
        let result8 = result as u8;
        let flags_lookup_index = make_flags_lookup_index(acc, value, result8);
        if result > 0xFF {
            flags |= FLAG_CARRY_MASK;
        }
        if result8 == 0 {
            flags |= FLAG_ZERO_MASK;
        }
        flags |= OVERFLOW_SUB_TABLE[flags_lookup_index];
        flags |= AUX_CARRY_SUB_TABLE[flags_lookup_index];
        (result8, flags)
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
                let (acc, flags) = Self::sub(
                    self.acc,
                    (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET,
                    old_flags);
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Subca => {
                // SUBC A
                // A <- A - CF
                let (acc, flags) = Self::sub(
                    self.acc,
                    (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET,
                    old_flags);
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Izsna => {
                // IZSN A
                let (acc, flags) = Self::add(self.acc, 1, old_flags);
                if flags & FLAG_ZERO_MASK != 0 {
                    pc_increment = 2;
                }
                self.acc = acc;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Dzsna => {
                // DZSN A
                let (acc, flags) = Self::sub(self.acc, 1, old_flags);
                if flags & FLAG_ZERO_MASK != 0 {
                    pc_increment = 2;
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
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                self.acc = !self.acc;
                if self.acc == 0 {
                    flags |= FLAG_ZERO_MASK;
                }
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Nega => {
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                self.acc = (!self.acc) + 1;
                if self.acc == 0 {
                    flags |= FLAG_ZERO_MASK;
                }
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Sra => {
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= (self.acc & 0x01) << FLAG_CARRY_OFFSET;
                self.acc >>= 1;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Sla => {
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= ((self.acc & 0x80) >> 7) << FLAG_CARRY_OFFSET;
                self.acc <<= 1;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Srca => {
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= (self.acc & 0x01) << FLAG_CARRY_OFFSET;
                self.acc >>= 1;
                self.acc |= ((old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET) << 7;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Slca => {
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                flags |= ((self.acc & 0x80) >> 7) << FLAG_CARRY_OFFSET;
                self.acc <<= 1;
                self.acc |= (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET;
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Swapa => {
                self.acc = ((self.acc & 0xF0) >> 4) | ((self.acc & 0x0F) << 4);
            },
            IrOpcode::Wdreset => {
                self.bus.wdt_reset();
            },
            IrOpcode::Pushaf => {
                let sp = self.bus.read_io(SP_IO_ADDR);
                self.bus.write_ram(sp, self.acc);
                self.bus.write_ram(sp.wrapping_add(1), self.bus.read_io(FLAGS_IO_ADDR));
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_add(2));
            },
            IrOpcode::Popaf => {
                let sp = self.bus.read_io(SP_IO_ADDR);
                self.bus.write_io(FLAGS_IO_ADDR, self.bus.read_ram(sp.wrapping_sub(1)));
                self.acc = self.bus.read_ram(sp.wrapping_sub(2));
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
            },
            IrOpcode::Reset => {
                self.bus.reset();
                // TODO : Implement correct reset behavior
            },
            IrOpcode::Stopsys => {
                self.bus.stop_sys();
            },
            IrOpcode::Stopexe => {
                self.bus.stop_exe()
            },
            IrOpcode::Engint => {
                self.global_interrupts = true;
            },
            IrOpcode::Disgint => {
                self.global_interrupts = false
            },
            IrOpcode::Ret => {
                let sp = self.bus.read_io(SP_IO_ADDR);
                let pc = ((self.bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
                    | (self.bus.read_ram(sp.wrapping_sub(2)) as u16);
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
                self.pc = pc;
                pc_increment = 0;
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Reti => {
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
                // No implementation for mul yet
            },
            IrOpcode::Xorioa => {
                self.bus.write_io(ir.operand8(), self.bus.read_io(ir.operand8()) ^ self.acc);
            },
            IrOpcode::Movioa => {
                self.bus.write_io(ir.operand8(), self.acc);
            },
            IrOpcode::Movaio => {
                let mut flags = old_flags & !FLAG_ZERO_MASK;
                self.acc = self.bus.read_io(ir.operand8());
                if self.acc == 0 {
                    flags |= FLAG_ZERO_MASK;
                }
                self.bus.write_io(FLAGS_IO_ADDR, flags);
            },
            IrOpcode::Stt16 => {
                let word = (self.bus.read_ram(ir.operand8()) as u16)
                    | ((self.bus.read_ram(ir.operand8()) as u16) << 8);
                self.bus.write_tim16(word);
            },
            IrOpcode::Ldt16 => {
                let word = self.bus.read_tim16();
                self.bus.write_ram(ir.operand8(), word as u8);
                self.bus.write_ram(ir.operand8().wrapping_add(1), (word >> 8) as u8);
            },
            IrOpcode::Idxmma => {
                let addr = (self.bus.read_ram(ir.operand8()) as u16)
                    | ((self.bus.read_ram(ir.operand8().wrapping_add(1)) as u16) << 8);
                self.bus.write_ram_far(addr, self.acc);
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Idxmam => {
                let addr = (self.bus.read_ram(ir.operand8()) as u16)
                    | ((self.bus.read_ram(ir.operand8().wrapping_add(1)) as u16) << 8);
                self.acc = self.bus.read_ram_far(addr);
                next_state = PdkCoreState::Skip;
            },
            IrOpcode::Retk => {
                self.acc = ir.operand16() as u8;
                let sp = self.bus.read_io(SP_IO_ADDR);
                let pc = ((self.bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
                    | (self.bus.read_ram(sp.wrapping_sub(2)) as u16);
                self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
                self.pc = pc;
                pc_increment = 0;
            },
            IrOpcode::T0snm => {
                if self.bus.read_ram(ir.operand8()) & (1 << ir.operand16() as u8) == 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::T1snm => {
                if self.bus.read_ram(ir.operand8()) & (1 << ir.operand16() as u8) != 0 {
                    pc_increment = 2;
                    next_state = PdkCoreState::Skip;
                }
            },
            IrOpcode::Set0m => {
                self.bus.write_ram(
                    ir.operand8(),
                    self.bus.read_ram(ir.operand8()) | (1 << ir.operand16() as u8));
            },
            IrOpcode::Set1m => {
                self.bus.write_ram(
                    ir.operand8(),
                    self.bus.read_ram(ir.operand8()) & (!(1 << ir.operand16() as u8)));
            },
            IrOpcode::Addma => {},
            IrOpcode::Subma => {},
            IrOpcode::Addcma => {},
            IrOpcode::Subcma => {},
            IrOpcode::Andma => {},
            IrOpcode::Orma => {},
            IrOpcode::Xorma => {},
            IrOpcode::Movma => {},
            IrOpcode::Addam => {},
            IrOpcode::Subam => {},
            IrOpcode::Addcam => {},
            IrOpcode::Subcam => {},
            IrOpcode::Andam => {},
            IrOpcode::Oram => {},
            IrOpcode::Xoram => {},
            IrOpcode::Movam => {},
            IrOpcode::Addcm => {},
            IrOpcode::Subcm => {},
            IrOpcode::Izsnm => {},
            IrOpcode::Dzsnm => {},
            IrOpcode::Incm => {},
            IrOpcode::Decm => {},
            IrOpcode::Clearm => {},
            IrOpcode::Xchm => {},
            IrOpcode::Notm => {},
            IrOpcode::Negm => {},
            IrOpcode::Srm => {},
            IrOpcode::Slm => {},
            IrOpcode::Srcm => {},
            IrOpcode::Slcm => {},
            IrOpcode::Ceqsnam => {},
            IrOpcode::T0snio => {},
            IrOpcode::T1snio => {},
            IrOpcode::Set0io => {},
            IrOpcode::Set1io => {},
            IrOpcode::Addak => {},
            IrOpcode::Subak => {},
            IrOpcode::Ceqsnak => {},
            IrOpcode::Andak => {},
            IrOpcode::Orak => {},
            IrOpcode::Xorak => {},
            IrOpcode::Movak => {},
            IrOpcode::Goto => {},
            IrOpcode::Call => {},
            _ => {},
        }
        self.pc.wrapping_add(pc_increment);
        next_state
    }
}