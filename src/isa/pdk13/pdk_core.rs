use super::{
    bus::{Bus, BusExt},
    ir::{IrOpcode, IrSlot},
    ops,
    regs::*,
    Byte, IoAddr, RamAddr, RomAddr, Word,
};

#[derive(Copy, Clone)]
enum PdkCoreState {
    Execute,
    Skip,
}

pub struct PdkCore {
    acc: Byte,
    pc: RomAddr,
    state: PdkCoreState,
    global_interrupts: bool,
    // Execute cycle varibles
    prev_flags: Byte,
    pc_increment: Word,
    next_state: PdkCoreState,
}

impl PdkCore {
    pub fn new() -> Self {
        Self {
            acc: 0,
            pc: 0,
            state: PdkCoreState::Execute,
            global_interrupts: false,
            prev_flags: 0,
            pc_increment: 0,
            next_state: PdkCoreState::Execute,
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) {
        self.state = match self.state {
            PdkCoreState::Execute => self.execute(bus),
            PdkCoreState::Skip => PdkCoreState::Execute,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.global_interrupts = false;
    }

    pub fn acc(&self) -> Byte {
        self.acc
    }

    pub fn pc(&self) -> RomAddr {
        self.pc
    }

    pub fn global_interrupts_enabled(&self) -> bool {
        self.global_interrupts
    }

    #[rustfmt::skip]
    fn execute(&mut self, bus: &mut impl Bus) -> PdkCoreState {
        self.prev_flags = bus.read_io(FLAGS_IO_ADDR);
        let ir = bus.read_rom(self.pc);
        self.next_state = PdkCoreState::Execute;
        self.pc_increment = 1;

        match ir.ir_opcode() {
            IrOpcode::Ldsptl => self.load_rom_word_indirect_sp_lo(bus),
            IrOpcode::Ldspth => self.load_rom_word_indirect_sp_hi(bus),
            IrOpcode::Addca => self.alu_acc_binary(ops::addc, 0, bus),
            IrOpcode::Subca => self.alu_acc_binary(ops::subc, 0, bus),
            IrOpcode::Izsna => self.inc_and_skip_next_if_zero_acc(bus),
            IrOpcode::Dzsna => self.dec_and_skip_next_if_zero_acc(bus),
            IrOpcode::Pcadda => self.add_acc_to_pc(),
            IrOpcode::Nota => self.alu_acc_unary(ops::not, bus),
            IrOpcode::Nega => self.alu_acc_unary(ops::neg, bus),
            IrOpcode::Sra => self.alu_acc_unary(ops::sr, bus),
            IrOpcode::Sla => self.alu_acc_unary(ops::sl, bus),
            IrOpcode::Srca => self.alu_acc_unary(ops::src, bus),
            IrOpcode::Slca => self.alu_acc_unary(ops::slc, bus),
            IrOpcode::Swapa => self.swap_acc_nibbles(),
            IrOpcode::Wdreset => self.reset_watchdog_timer(bus),
            IrOpcode::Pushaf => self.push_af(bus),
            IrOpcode::Popaf => self.pop_af(bus),
            IrOpcode::Reset => self.software_reset(bus),
            IrOpcode::Stopsys => bus.stop_sys(),
            IrOpcode::Stopexe => bus.stop_exe(),
            IrOpcode::Engint => self.global_interrupts = true,
            IrOpcode::Disgint => self.global_interrupts = false,
            IrOpcode::Ret => self.ret(bus),
            IrOpcode::Reti => self.reti(bus),
            IrOpcode::Xorioa => self.xor_io_with_acc(ir.io_address(), bus),
            IrOpcode::Movioa => bus.write_io(ir.io_address(), self.acc),
            IrOpcode::Movaio => self.alu_acc_binary(ops::mov, bus.read_io(ir.io_address()), bus),
            IrOpcode::Stt16 => bus.write_tim16(bus.read_ram_word(ir.mem_address())),
            IrOpcode::Ldt16 => bus.write_ram_word(ir.mem_address(), bus.read_tim16()),
            IrOpcode::Idxmma => self.indirect_store_acc(ir.mem_address(), bus),
            IrOpcode::Idxmam => self.indirect_load_acc(ir.mem_address(), bus),
            IrOpcode::Retk => self.ret_immediate(ir.immediate() as u8, bus),
            IrOpcode::T0snm => self.skip_if_bit_clear_ram(ir.mem_address(), ir.bit_index() as u8, bus),
            IrOpcode::T1snm => self.skip_if_bit_set_ram(ir.mem_address(), ir.bit_index() as u8, bus),
            IrOpcode::Set0m => self.clear_bit_ram(ir.mem_address(), ir.bit_index() as u8, bus),
            IrOpcode::Set1m => self.set_bit_ram(ir.mem_address(), ir.bit_index() as u8, bus),
            IrOpcode::Addma => self.alu_mem_binary(ops::add, ir.mem_address(), self.acc, bus),
            IrOpcode::Subma => self.alu_mem_binary(ops::sub, ir.mem_address(), self.acc, bus),
            IrOpcode::Addcma => self.alu_mem_binary(ops::addc, ir.mem_address(), self.acc, bus),
            IrOpcode::Subcma => self.alu_mem_binary(ops::subc, ir.mem_address(), self.acc, bus),
            IrOpcode::Andma => self.alu_mem_binary(ops::and, ir.mem_address(), self.acc, bus),
            IrOpcode::Orma => self.alu_mem_binary(ops::or, ir.mem_address(), self.acc, bus),
            IrOpcode::Xorma => self.alu_mem_binary(ops::xor, ir.mem_address(), self.acc, bus),
            IrOpcode::Movma => bus.write_ram(ir.mem_address(), self.acc),
            IrOpcode::Addam => self.alu_acc_binary(ops::add, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Subam => self.alu_acc_binary(ops::sub, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Addcam => self.alu_acc_binary(ops::addc, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Subcam => self.alu_acc_binary(ops::subc, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Andam => self.alu_acc_binary(ops::and, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Oram => self.alu_acc_binary(ops::or, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Xoram => self.alu_acc_binary(ops::xor, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Movam => self.alu_acc_binary(ops::mov, bus.read_ram(ir.mem_address()), bus),
            IrOpcode::Addcm => self.alu_mem_binary(ops::addc, ir.mem_address(), 0, bus),
            IrOpcode::Subcm => self.alu_mem_binary(ops::subc, ir.mem_address(), 0, bus),
            IrOpcode::Izsnm => self.inc_and_skip_next_if_zero_ram(ir.mem_address(), bus),
            IrOpcode::Dzsnm => self.dec_and_skip_next_if_zero_ram(ir.mem_address(), bus),
            IrOpcode::Incm => self.alu_mem_binary(ops::add, ir.mem_address(), 1, bus),
            IrOpcode::Decm => self.alu_mem_binary(ops::sub, ir.mem_address(), 1, bus),
            IrOpcode::Clearm => bus.write_ram(ir.mem_address(), 0),
            IrOpcode::Xchm => self.exchange_acc_with_ram(ir.mem_address(), bus),
            IrOpcode::Notm => self.alu_mem_unary(ops::not, ir.mem_address(), bus),
            IrOpcode::Negm => self.alu_mem_unary(ops::neg, ir.mem_address(), bus),
            IrOpcode::Srm => self.alu_mem_unary(ops::sr, ir.mem_address(), bus),
            IrOpcode::Slm => self.alu_mem_unary(ops::sl, ir.mem_address(), bus),
            IrOpcode::Srcm => self.alu_mem_unary(ops::src, ir.mem_address(), bus),
            IrOpcode::Slcm => self.alu_mem_unary(ops::slc, ir.mem_address(), bus),
            IrOpcode::Ceqsnam => self.skip_next_if_equal(bus.read_ram(ir.mem_address()), bus),
            IrOpcode::T0snio => self.skip_if_bit_clear_io(ir.io_address(), ir.bit_index() as u8, bus),
            IrOpcode::T1snio => self.skip_if_bit_set_io(ir.io_address(), ir.bit_index() as u8, bus),
            IrOpcode::Set0io => self.clear_bit_io(ir.io_address(), ir.bit_index() as u8, bus),
            IrOpcode::Set1io => self.set_bit_io(ir.io_address(), ir.bit_index() as u8, bus),
            IrOpcode::Addak => self.alu_acc_binary(ops::add, ir.immediate() as u8, bus),
            IrOpcode::Subak => self.alu_acc_binary(ops::sub, ir.immediate() as u8, bus),
            IrOpcode::Ceqsnak => self.skip_next_if_equal(ir.immediate() as u8, bus),
            IrOpcode::Andak => self.alu_acc_binary(ops::and, ir.immediate() as u8, bus),
            IrOpcode::Orak => self.alu_acc_binary(ops::or, ir.immediate() as u8, bus),
            IrOpcode::Xorak => self.alu_acc_binary(ops::xor, ir.immediate() as u8, bus),
            IrOpcode::Movak => self.acc = ir.immediate() as u8,
            IrOpcode::Goto => self.goto(ir.rom_address()),
            IrOpcode::Call => self.call(ir.rom_address(), bus),
            _ => {}
        }
        self.pc = self.pc.wrapping_add(self.pc_increment);
        self.next_state
    }

    fn alu_acc_binary(
        &mut self,
        operation: ops::BinaryOperation,
        operand: Byte,
        bus: &mut impl Bus,
    ) {
        let (r, f) = operation(self.acc, operand, self.prev_flags);
        self.acc = r;
        bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn alu_mem_binary(
        &mut self,
        operation: ops::BinaryOperation,
        addr: Byte,
        operand: Byte,
        bus: &mut impl Bus,
    ) {
        let (r, f) = operation(bus.read_ram(addr), operand, self.prev_flags);
        bus.write_ram(addr, r);
        bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn alu_acc_unary(&mut self, operation: ops::UnaryOperation, bus: &mut impl Bus) {
        let (r, f) = operation(self.acc, self.prev_flags);
        self.acc = r;
        bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn alu_mem_unary(&mut self, operation: ops::UnaryOperation, addr: Byte, bus: &mut impl Bus) {
        let (r, f) = operation(bus.read_ram(addr), self.prev_flags);
        bus.write_ram(addr, r);
        bus.write_io(FLAGS_IO_ADDR, f);
    }

    fn ret(&mut self, bus: &mut impl Bus) {
        let sp = bus.read_io(SP_IO_ADDR);
        let pc = ((bus.read_ram(sp.wrapping_sub(1)) as u16) << 8)
            | (bus.read_ram(sp.wrapping_sub(2)) as u16);
        bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
        self.pc = pc;
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn reti(&mut self, bus: &mut impl Bus) {
        self.ret(bus);
        self.global_interrupts = true;
    }

    fn ret_immediate(&mut self, value: Byte, bus: &mut impl Bus) {
        self.acc = value;
        self.ret(bus);
    }

    fn goto(&mut self, addr: Word) {
        self.pc = addr;
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn call(&mut self, addr: Word, bus: &mut impl Bus) {
        let sp = bus.read_io(SP_IO_ADDR);
        bus.write_ram_word(sp, self.pc.wrapping_add(1));
        self.pc = addr;
        bus.write_io(SP_IO_ADDR, sp.wrapping_add(2));
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn set_bit_ram(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        bus.write_ram(addr, bus.read_ram(addr) | (1 << bit));
    }

    fn clear_bit_ram(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        bus.write_ram(addr, bus.read_ram(addr) & !(1 << bit));
    }

    fn set_bit_io(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        bus.write_io(addr, bus.read_io(addr) | (1 << bit));
    }

    fn clear_bit_io(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        bus.write_io(addr, bus.read_io(addr) & !(1 << bit));
    }

    fn skip_if_bit_set_io(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        if bus.read_io(addr) & (1 << bit as u8) != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn skip_if_bit_clear_io(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        if bus.read_io(addr) & (1 << bit) == 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn skip_if_bit_set_ram(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        if bus.read_ram(addr) & (1 << bit) != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn skip_if_bit_clear_ram(&mut self, addr: u8, bit: u8, bus: &mut impl Bus) {
        if bus.read_ram(addr) & (1 << bit) == 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }

    fn exchange_acc_with_ram(&mut self, addr: u8, bus: &mut impl Bus) {
        let tmp = self.acc;
        self.acc = bus.read_ram(addr);
        bus.write_ram(addr, tmp);
    }

    fn swap_acc_nibbles(&mut self) {
        self.acc = ((self.acc & 0xF0) >> 4) | ((self.acc & 0x0F) << 4);
    }

    fn add_acc_to_pc(&mut self) {
        self.pc = self.pc.wrapping_add(self.acc as u16);
        self.pc_increment = 0;
        self.next_state = PdkCoreState::Skip;
    }

    fn push_af(&mut self, bus: &mut impl Bus) {
        let sp = bus.read_io(SP_IO_ADDR);
        bus.write_ram(sp, self.acc);
        bus.write_ram(sp.wrapping_add(1), bus.read_io(FLAGS_IO_ADDR));
        bus.write_io(SP_IO_ADDR, sp.wrapping_add(2));
    }

    fn pop_af(&mut self, bus: &mut impl Bus) {
        let sp = bus.read_io(SP_IO_ADDR);
        bus.write_io(FLAGS_IO_ADDR, bus.read_ram(sp.wrapping_sub(1)));
        self.acc = bus.read_ram(sp.wrapping_sub(2));
        bus.write_io(SP_IO_ADDR, sp.wrapping_sub(2));
    }

    fn software_reset(&mut self, bus: &mut impl Bus) {
        bus.reset();
        self.reset();
        self.pc_increment = 0;
    }

    fn reset_watchdog_timer(&mut self, bus: &mut impl Bus) {
        bus.wdt_reset();
    }

    fn load_rom_word_indirect_sp_hi(&mut self, bus: &mut impl Bus) {
        let sp = bus.read_io(SP_IO_ADDR);
        let addr = bus.read_ram(sp) as u16 | ((bus.read_ram(sp.wrapping_add(1)) as u16) << 8);
        self.acc = (bus.read_rom(addr).original_word() >> 8) as u8
    }

    fn load_rom_word_indirect_sp_lo(&mut self, bus: &mut impl Bus) {
        let sp = bus.read_io(SP_IO_ADDR);
        let addr = bus.read_ram(sp) as u16 | ((bus.read_ram(sp.wrapping_add(1)) as u16) << 8);
        self.acc = bus.read_rom(addr).original_word() as u8
    }

    fn inc_and_skip_next_if_zero_acc(&mut self, bus: &mut impl Bus) {
        let (acc, flags) = ops::add(self.acc, 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        self.acc = acc;
        bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn dec_and_skip_next_if_zero_acc(&mut self, bus: &mut impl Bus) {
        let (acc, flags) = ops::sub(self.acc, 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        self.acc = acc;
        bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn inc_and_skip_next_if_zero_ram(&mut self, addr: Byte, bus: &mut impl Bus) {
        let (acc, flags) = ops::add(bus.read_ram(addr), 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        bus.write_ram(addr, acc);
        bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn dec_and_skip_next_if_zero_ram(&mut self, addr: Byte, bus: &mut impl Bus) {
        let (acc, flags) = ops::sub(bus.read_ram(addr), 1, self.prev_flags);
        if flags & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
        bus.write_ram(addr, acc);
        bus.write_io(FLAGS_IO_ADDR, flags);
    }

    fn xor_io_with_acc(&mut self, addr: Byte, bus: &mut impl Bus) {
        bus.write_io(addr, bus.read_io(addr) ^ self.acc);
    }

    fn indirect_store_acc(&mut self, addr: Byte, bus: &mut impl Bus) {
        bus.write_ram(bus.read_ram_word(addr) as Byte, self.acc);
        self.next_state = PdkCoreState::Skip;
    }

    fn indirect_load_acc(&mut self, addr: Byte, bus: &mut impl Bus) {
        self.acc = bus.read_ram(bus.read_ram_word(addr) as Byte);
        self.next_state = PdkCoreState::Skip;
    }

    fn skip_next_if_equal(&mut self, value: Byte, bus: &mut impl Bus) {
        let (_r, f) = ops::sub(self.acc, value, self.prev_flags);
        bus.write_io(FLAGS_IO_ADDR, f);
        if f & FLAG_ZERO_MASK != 0 {
            self.pc_increment = 2;
            self.next_state = PdkCoreState::Skip;
        }
    }
}
