use super::{
    IoAddr, Byte, RamAddr, RomAddr, Word,
    ir::{IrSlot, IrOpcode},
    ops,
    bus::{Bus, BusExt},
    regs::*,
};


#[derive(Copy, Clone)]
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
    // Execute cycle varibles
    prev_flags: Byte,
    pc_increment: Word,
    next_state: PdkCoreState,
}

impl<B: Bus> PdkCore<B> {
    pub fn new(bus: B) -> Self {
        Self {
            acc: 0,
            pc: 0,
            state: PdkCoreState::Execute,
            global_interrupts: false,
            bus,
            prev_flags: 0,
            pc_increment: 0,
            next_state: PdkCoreState::Execute,
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

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.global_interrupts = false;
    }

    fn execute(&mut self) -> PdkCoreState {
        self.prev_flags = self.bus().read_io(FLAGS_IO_ADDR);
        let ir = self.bus.read_ir(self.pc);
        self.next_state = PdkCoreState::Execute;
        self.pc_increment = 1;

        match ir.opcode() {
            IrOpcode::Ldsptl => self.load_rom_word_indirect_sp_lo(),
            IrOpcode::Ldspth => self.load_rom_word_indirect_sp_hi(),
            IrOpcode::Addca => self.alu_acc_binary(ops::addc, 0),
            IrOpcode::Subca => self.alu_acc_binary(ops::subc, 0),
            IrOpcode::Izsna => self.inc_and_skip_next_if_zero_acc(),
            IrOpcode::Dzsna => self.dec_and_skip_next_if_zero_acc(),
            IrOpcode::Pcadda => self.add_acc_to_pc(),
            IrOpcode::Nota => self.alu_acc_unary(ops::not),
            IrOpcode::Nega => self.alu_acc_unary(ops::neg),
            IrOpcode::Sra => self.alu_acc_unary(ops::sr),
            IrOpcode::Sla => self.alu_acc_unary(ops::sl),
            IrOpcode::Srca => self.alu_acc_unary(ops::src),
            IrOpcode::Slca => self.alu_acc_unary(ops::slc),
            IrOpcode::Swapa => self.swap_acc_nibbles(),
            IrOpcode::Wdreset => self.reset_watchdog_timer(),
            IrOpcode::Pushaf => self.push_af(),
            IrOpcode::Popaf => self.pop_af(),
            IrOpcode::Reset => self.software_reset(),
            IrOpcode::Stopsys => self.bus.stop_sys(),
            IrOpcode::Stopexe => self.bus.stop_exe(),
            IrOpcode::Engint => self.global_interrupts = true,
            IrOpcode::Disgint => self.global_interrupts = false,
            IrOpcode::Ret => self.ret(),
            IrOpcode::Reti => self.reti(),
            IrOpcode::Xorioa => self.xor_io_with_acc(ir.operand8()),
            IrOpcode::Movioa => self.bus.write_io(ir.operand8(), self.acc),
            IrOpcode::Movaio => self.alu_acc_binary(ops::mov, self.bus.read_io(ir.operand8())),
            IrOpcode::Stt16 => self.bus.write_tim16(self.bus.read_ram_word(ir.operand8())),
            IrOpcode::Ldt16 => self.bus.write_ram_word(ir.operand8(), self.bus.read_tim16()),
            IrOpcode::Idxmma => self.indirect_store_acc(ir.operand8()),
            IrOpcode::Idxmam => self.indirect_load_acc(ir.operand8()),
            IrOpcode::Retk => self.ret_immediate(ir.operand16() as u8),
            IrOpcode::T0snm => self.skip_next_if_bit_clear_ram(ir.operand8(), ir.operand16() as u8),
            IrOpcode::T1snm => self.skip_next_if_bit_set_ram(ir.operand8(), ir.operand16() as u8),
            IrOpcode::Set0m => self.clear_bit_ram(ir.operand8(), ir.operand16() as u8),
            IrOpcode::Set1m => self.set_bit_ram(ir.operand8(), ir.operand16() as u8),
            IrOpcode::Addma => self.alu_mem_binary(ops::add, ir.operand8(), self.acc),
            IrOpcode::Subma => self.alu_mem_binary(ops::sub, ir.operand8(), self.acc),
            IrOpcode::Addcma => self.alu_mem_binary(ops::addc, ir.operand8(), self.acc),
            IrOpcode::Subcma => self.alu_mem_binary(ops::subc, ir.operand8(), self.acc),
            IrOpcode::Andma => self.alu_mem_binary(ops::and, ir.operand8(), self.acc),
            IrOpcode::Orma => self.alu_mem_binary(ops::or, ir.operand8(), self.acc),
            IrOpcode::Xorma => self.alu_mem_binary(ops::xor, ir.operand8(), self.acc),
            IrOpcode::Movma => self.bus.write_io(ir.operand8(), self.acc),
            IrOpcode::Addam => self.alu_acc_binary(ops::add, self.bus.read_ram(ir.operand8())),
            IrOpcode::Subam => self.alu_acc_binary(ops::sub, self.bus.read_ram(ir.operand8())),
            IrOpcode::Addcam => self.alu_acc_binary(ops::addc, self.bus.read_ram(ir.operand8())),
            IrOpcode::Subcam => self.alu_acc_binary(ops::subc, self.bus.read_ram(ir.operand8())),
            IrOpcode::Andam => self.alu_acc_binary(ops::and, self.bus.read_ram(ir.operand8())),
            IrOpcode::Oram => self.alu_acc_binary(ops::or, self.bus.read_ram(ir.operand8())),
            IrOpcode::Xoram => self.alu_acc_binary(ops::xor, self.bus.read_ram(ir.operand8())),
            IrOpcode::Movam => self.alu_acc_binary(ops::mov, self.bus.read_ram(ir.operand8())),
            IrOpcode::Addcm => self.alu_mem_binary(ops::addc, 0, self.acc),
            IrOpcode::Subcm => self.alu_mem_binary(ops::subc, 0, self.acc),
            IrOpcode::Izsnm => self.inc_and_skip_next_if_zero_ram(ir.operand8()),
            IrOpcode::Dzsnm => self.dec_and_skip_next_if_zero_ram(ir.operand8()),
            IrOpcode::Incm => self.alu_mem_binary(ops::add, ir.operand8(), 1),
            IrOpcode::Decm => self.alu_mem_binary(ops::sub, ir.operand8(), 1),
            IrOpcode::Clearm => self.bus.write_ram(ir.operand8(), 0),
            IrOpcode::Xchm => self.exchange_acc_with_ram(ir.operand8()),
            IrOpcode::Notm => self.alu_mem_unary(ops::not, ir.operand8()),
            IrOpcode::Negm => self.alu_mem_unary(ops::neg, ir.operand8()),
            IrOpcode::Srm => self.alu_mem_unary(ops::sr, ir.operand8()),
            IrOpcode::Slm => self.alu_mem_unary(ops::sl, ir.operand8()),
            IrOpcode::Srcm => self.alu_mem_unary(ops::src, ir.operand8()),
            IrOpcode::Slcm => self.alu_mem_unary(ops::slc, ir.operand8()),
            IrOpcode::Ceqsnam => self.skip_next_if_equal(self.bus.read_ram(ir.operand8())),
            IrOpcode::T0snio => self.skip_next_if_bit_clear_io(ir.operand8(), ir.operand16() as u8),
            IrOpcode::T1snio => self.skip_next_if_bit_set_io(ir.operand8(), ir.operand16() as u8),
            IrOpcode::Set0io => self.clear_bit_io(ir.operand8(), ir.operand16() as u8),
            IrOpcode::Set1io => self.set_bit_io(ir.operand8(), ir.operand16() as u8),
            IrOpcode::Addak => self.alu_acc_binary(ops::add, ir.operand16() as u8),
            IrOpcode::Subak => self.alu_acc_binary(ops::sub, ir.operand16() as u8),
            IrOpcode::Ceqsnak => self.skip_next_if_equal(ir.operand16() as u8),
            IrOpcode::Andak => self.alu_acc_binary(ops::and, ir.operand16() as u8),
            IrOpcode::Orak => self.alu_acc_binary(ops::or, ir.operand16() as u8),
            IrOpcode::Xorak => self.alu_acc_binary(ops::xor, ir.operand16() as u8),
            IrOpcode::Movak => self.acc = ir.operand16() as u8,
            IrOpcode::Goto => self.goto(ir.operand16()),
            IrOpcode::Call => self.call(ir.operand16()),
            _ => {},
        }
        self.pc = self.pc.wrapping_add(self.pc_increment);
        self.next_state
    }

    fn alu_acc_binary(&mut self, operation: ops::BinaryOperation, operand: Byte) {
        let (r, f) = operation(self.acc, operand, self.prev_flags);
        self.acc = r;
        self.bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn alu_mem_binary(&mut self, operation: ops::BinaryOperation, addr: Byte, operand: Byte) {
        let (r, f) = operation(self.bus.read_ram(addr), operand, self.prev_flags);
        self.bus.write_ram(addr, r);
        self.bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn alu_acc_unary(&mut self, operation: ops::UnaryOperation) {
        let (r, f) = operation(self.acc, self.prev_flags);
        self.acc = r;
        self.bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn alu_mem_unary(&mut self, operation: ops::UnaryOperation, addr: Byte) {
        let (r, f) = operation(self.bus.read_ram(addr), self.prev_flags);
        self.bus.write_ram(addr, r);
        self.bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn ret(&mut self) {
        let sp = self.bus.read_io(SP_IO_ADDR);
        let pc = ((self.bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
            | (self.bus.read_ram(sp.wrapping_sub(2)) as u16);
        self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
        self.pc = pc;
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn reti(&mut self) {
        self.ret();
        self.global_interrupts = true;
    }

    fn ret_immediate(&mut self, value: Byte) {
        self.acc = value;
        self.ret();
    }

    fn goto(&mut self, addr: Word) {
        self.pc = addr;
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn call(&mut self, addr: Word) {
        let sp = self.bus.read_io(SP_IO_ADDR);
        self.bus.write_ram_word(sp, self.pc.wrapping_add(1));
        self.pc = addr;
        self.bus.write_io(SP_IO_ADDR, sp.wrapping_add(2));
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn set_bit_ram(&mut self, addr: u8, bit: u8) {
        self.bus.write_ram(addr, self.bus.read_ram(addr) | (1 << bit));
    }

    fn clear_bit_ram(&mut self, addr: u8, bit: u8) {
        self.bus.write_ram(addr, self.bus.read_ram(addr) & !(1 << bit));
    }

    fn set_bit_io(&mut self, addr: u8, bit: u8) {
        self.bus.write_io(addr, self.bus.read_io(addr) | (1 << bit));
    }

    fn clear_bit_io(&mut self, addr: u8, bit: u8) {
        self.bus.write_io(addr, self.bus.read_io(addr) & !(1 << bit));
    }

    fn skip_next_if_bit_set_io(&mut self, addr: u8, bit: u8) {
        if self.bus.read_io(addr) & (1 << bit as u8) != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn skip_next_if_bit_clear_io(&mut self, addr: u8, bit: u8) {
        if self.bus.read_io(addr) & (1 << bit) == 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn skip_next_if_bit_set_ram(&mut self, addr: u8, bit: u8) {
        if self.bus.read_ram(addr) & (1 << bit) != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn skip_next_if_bit_clear_ram(&mut self, addr: u8, bit: u8) {
        if self.bus.read_ram(addr) & (1 << bit) == 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn exchange_acc_with_ram(&mut self, addr: u8) {
        let tmp = self.acc;
        self.acc = self.bus.read_ram(addr);
        self.bus.write_ram(addr, tmp);
    }

    fn swap_acc_nibbles(&mut self) {
        self.acc = ((self.acc & 0xF0) >> 4) | ((self.acc & 0x0F) << 4);
    }

    fn add_acc_to_pc(&mut self) {
        self.pc = self.pc.wrapping_add(self.acc as u16);
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn push_af(&mut self) {
        let sp = self.bus.read_io(SP_IO_ADDR);
        self.bus.write_ram(sp.wrapping_add(1), self.acc);
        self.bus.write_ram(sp, self.bus.read_io(FLAGS_IO_ADDR));
        self.bus.write_io(SP_IO_ADDR, sp.wrapping_add(2));
    }

    fn pop_af(&mut self) {
        let sp = self.bus.read_io(SP_IO_ADDR);
        self.acc = self.bus.read_ram(sp.wrapping_sub(1));
        self.bus.write_io(FLAGS_IO_ADDR, self.bus.read_ram(sp.wrapping_sub(2)));
        self.bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
    }

    fn software_reset(&mut self) {
        self.bus.reset();
        self.reset();
        self.pc_increment = 0;
    }

    fn reset_watchdog_timer(&mut self) {
        self.bus.wdt_reset();
    }

    fn load_rom_word_indirect_sp_hi(&mut self) {
        let sp = self.bus.read_io(SP_IO_ADDR);
        let addr = self.bus.read_ram(sp) as u16 |
            ((self.bus.read_ram(sp.wrapping_add(1)) as u16) << 8);
        self.acc = (self.bus.read_rom(addr) >> 8) as u8
    }

    fn load_rom_word_indirect_sp_lo(&mut self) {
        let sp = self.bus.read_io(SP_IO_ADDR);
        let addr = self.bus.read_ram(sp) as u16 |
            ((self.bus.read_ram(sp.wrapping_add(1)) as u16) << 8);
        self.acc = self.bus.read_rom(addr) as u8
    }

    fn inc_and_skip_next_if_zero_acc(&mut self) {
        let (acc, flags) = ops::add(self.acc, 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        self.acc = acc;
        self.bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn dec_and_skip_next_if_zero_acc(&mut self) {
        let (acc, flags) = ops::sub(self.acc, 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        self.acc = acc;
        self.bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn inc_and_skip_next_if_zero_ram(&mut self, addr: Byte) {
        let (acc, flags) = ops::add(self.bus.read_ram(addr), 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        self.bus.write_ram(addr, acc);
        self.bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn dec_and_skip_next_if_zero_ram(&mut self, addr: Byte) {
        let (acc, flags) = ops::sub(self.bus.read_ram(addr), 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        self.bus.write_ram(addr, acc);
        self.bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn xor_io_with_acc(&mut self, addr: Byte) {
        self.bus.write_io(addr, self.bus.read_io(addr) ^ self.acc);
    }

    fn indirect_store_acc(&mut self, addr: Byte) {
        self.bus.write_ram(self.bus.read_ram_word(addr) as Byte, self.acc);
        self.next_state = PdkCoreState::Skip;
    }

    fn indirect_load_acc(&mut self, addr: Byte) {
        self.acc = self.bus.read_ram(self.bus.read_ram_word(addr) as Byte);
        self.next_state = PdkCoreState::Skip;
    }

    fn skip_next_if_equal(&mut self, value: Byte) {
        let (r, f) = ops::sub(self.acc, value, self.prev_flags);
        self.bus.write_io(FLAGS_IO_ADDR, f);
        if f & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }
}