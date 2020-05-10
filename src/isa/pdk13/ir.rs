//! This module introduces ir code for instruction emulation; This code is required to avoid
//! instruction group decoding on each instruction execution. Using of intermediate id for the
//! instruction (IrOpcode) makes eassy for the compiler to make compact jump table. This
//! optimization potentially allows emulator run the emulator with the reasonable speed and
//! reasonable ROM footprint (Which is crucial for the constrained devices such as MCU)
//!
//! 32 bit IR structure
//! -------8-----------13--3-------8
//! ccccccccwwwwwwwwwwwwwyyyxxxxxxxx
//!        c            w  y       x
//! c => Ir instruction id
//! w => original 13 bit word
//! y => hi operand part (used alone as bit index)
//! x => lo operand part (used alone as mem/io address)
//! x | y => used together as 10 bit address

use super::{limit, opcode_stamp::OpcodeStamp, Pdk13Error, Pdk13Result, Word};

#[derive(Copy, Clone)]
pub struct IrSlot(u32);

impl IrSlot {
    pub fn from_instruction(instruction: Word) -> Pdk13Result<Self> {
        generate_ir(instruction)
    }

    pub fn ir_opcode(&self) -> IrOpcode {
        IrOpcode::from_primitive(((self.0 >> 24) & 0x000000FF) as u8)
    }

    pub fn original_word(&self) -> u16 {
        ((self.0 >> 11) & 0x00001FFF) as u16
    }

    pub fn mem_address(&self) -> u8 {
        self.operand8()
    }

    pub fn io_address(&self) -> u8 {
        self.operand8()
    }

    pub fn bit_index(&self) -> u8 {
        self.operand3()
    }

    pub fn immediate(&self) -> u8 {
        self.operand8()
    }

    pub fn rom_address(&self) -> u16 {
        self.operand11()
    }

    fn operand3(&self) -> u8 {
        ((self.0 >> 8) & 0x00000007) as u8
    }

    fn operand8(&self) -> u8 {
        (self.0 & 0x000000FF) as u8
    }

    fn operand11(&self) -> u16 {
        (self.0 & 0x000007FF) as u16
    }
}

impl Default for IrSlot {
    fn default() -> Self {
        Self(0)
    }
}

pub struct IrSlotBuilder(u32);

impl IrSlotBuilder {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn original_word(&mut self, value: u16) -> &mut Self {
        self.0 &= !(0x1FFF << 11);
        self.0 |= ((value & 0x1FFF) as u32) << 11;
        self
    }

    pub fn ir_opcode(&mut self, value: IrOpcode) -> &mut Self {
        self.0 &= !(0xFF << 24);
        self.0 |= (value as u32) << 24;
        self
    }

    pub fn mem_address(&mut self, value: u8) -> &mut Self {
        self.operand8(value);
        self
    }

    pub fn io_address(&mut self, value: u8) -> &mut Self {
        self.operand8(value);
        self
    }

    pub fn bit_index(&mut self, value: u8) -> &mut Self {
        self.operand3(value);
        self
    }

    pub fn immediate(&mut self, value: u8) -> &mut Self {
        self.operand8(value);
        self
    }

    pub fn rom_address(&mut self, value: u16) -> &mut Self {
        self.operand11(value);
        self
    }

    fn operand3(&mut self, value: u8) -> &mut Self {
        self.0 &= !(0x07 << 8);
        self.0 |= ((value & 0x07) as u32) << 8;
        self
    }

    fn operand8(&mut self, value: u8) -> &mut Self {
        self.0 &= !0xFF;
        self.0 |= value as u32;
        self
    }

    fn operand11(&mut self, value: u16) -> &mut Self {
        self.0 &= !0x7FF;
        self.0 |= (value & 0x7FF) as u32;
        self
    }

    pub fn build(&mut self) -> IrSlot {
        IrSlot(self.0)
    }
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum IrOpcode {
    Nop,
    Ldsptl,
    Ldspth,
    Addca,
    Subca,
    Izsna,
    Dzsna,
    Pcadda,
    Nota,
    Nega,
    Sra,
    Sla,
    Srca,
    Slca,
    Swapa,
    Wdreset,
    Pushaf,
    Popaf,
    Reset,
    Stopsys,
    Stopexe,
    Engint,
    Disgint,
    Ret,
    Reti,
    Mul,
    Xorioa,
    Movioa,
    Movaio,
    Stt16,
    Ldt16,
    Idxmma,
    Idxmam,
    Retk,
    T0snm,
    T1snm,
    Set0m,
    Set1m,
    Addma,
    Subma,
    Addcma,
    Subcma,
    Andma,
    Orma,
    Xorma,
    Movma,
    Addam,
    Subam,
    Addcam,
    Subcam,
    Andam,
    Oram,
    Xoram,
    Movam,
    Addcm,
    Subcm,
    Izsnm,
    Dzsnm,
    Incm,
    Decm,
    Clearm,
    Xchm,
    Notm,
    Negm,
    Srm,
    Slm,
    Srcm,
    Slcm,
    Ceqsnam,
    T0snio,
    T1snio,
    Set0io,
    Set1io,
    Addak,
    Subak,
    Ceqsnak,
    Andak,
    Orak,
    Xorak,
    Movak,
    Goto,
    Call,
}

impl IrOpcode {
    // Function is perfectly safe as long as IrOpcode is non-exhaustive
    fn from_primitive(value: u8) -> IrOpcode {
        unsafe { core::mem::transmute(value) }
    }
}

pub(crate) fn generate_ir(instruction: Word) -> Pdk13Result<IrSlot> {
    if !limit::is_valid_opcode(instruction) {
        return Err(Pdk13Error::TooBigWord(instruction));
    }

    const MISC_GROUP_MASK: u16 = 0b111_1111111000000;
    const MISC_GROUP_STAMP: u16 = 0b000_0000000000000;

    const XOR_IO_GROUP_MASK: u16 = 0b111_1111111000000;
    const XOR_IO_GROUP_STAMP: u16 = 0b000_0000001000000;
    const XOR_IO_OPCODE_MASK: u16 = 0b111_1111111100000;

    const MOV_IO_GROUP_MASK: u16 = 0b111_1111111000000;
    const MOV_IO_GROUP_STAMP: u16 = 0b000_0000010000000;
    const MOV_IO_OPCODE_MASK: u16 = 0b111_1111111100000;

    const MEM16_GROUP_MASK: u16 = 0b111_1111111000000;
    const MEM16_GROUP_STAMP: u16 = 0b000_0000011000000;
    const MEM16_OPCODE_MASK: u16 = 0b111_1111111100001;

    const RET_CONST_GROUP_MASK: u16 = 0b111_1111100000000;
    const RET_CONST_GROUP_STAMP: u16 = 0b000_0000100000000;
    const RET_CONST_OPCODE_MASK: u16 = 0b111_1111100000000;

    const MEM_BIT_OPS_GROUP_MASK: u16 = 0b111_1111000000000;
    const MEM_BIT_OPS_GROUP_STAMP: u16 = 0b000_0001000000000;
    const MEM_BIT_OPS_OPCODE_MASK: u16 = 0b111_1111100010000;

    const MEM_AND_ACC_GROUP_MASK: u16 = 0b111_1110000000000;
    const MEM_AND_ACC_GROUP_STAMP: u16 = 0b000_0010000000000;
    const MEM_AND_ACC_OPCODE_MASK: u16 = 0b111_1111111000000;

    const MEM_GROUP_MASK: u16 = 0b111_1110000000000;
    const MEM_GROUP_STAMP: u16 = 0b000_0100000000000;
    const MEM_OPCODE_MASK: u16 = 0b111_1111111000000;

    const IO_BIT_OPS_GROUP_MASK: u16 = 0b111_1110000000000;
    const IO_BIT_OPS_GROUP_STAMP: u16 = 0b000_0110000000000;
    const IO_BIT_OPS_OPCODE_MASK: u16 = 0b111_1111100000000;

    const ACC_CONST_GROUP_MASK: u16 = 0b111_1100000000000;
    const ACC_CONST_GROUP_STAMP: u16 = 0b000_1000000000000;
    const ACC_CONST_OPCODE_MASK: u16 = 0b111_1111100000000;

    const JUMP_GROUP_MASK: u16 = 0b111_1100000000000;
    const JUMP_GROUP_STAMP: u16 = 0b000_1100000000000;
    const JUMP_OPCODE_MASK: u16 = 0b111_1110000000000;

    let mut ir_builder = IrSlotBuilder::new();
    ir_builder.original_word(instruction);
    let opcode_stamp;

    if instruction & MISC_GROUP_MASK == MISC_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction);
    // No operands
    } else if instruction & XOR_IO_GROUP_MASK == XOR_IO_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & XOR_IO_OPCODE_MASK);
        // Operand 1: 5 bit io address at offset 0
        ir_builder.io_address((instruction & 0b11111) as u8);
    } else if instruction & MOV_IO_GROUP_MASK == MOV_IO_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MOV_IO_OPCODE_MASK);
        // Operand 1: 5 bit io address at offset 0
        ir_builder.io_address((instruction & 0b11111) as u8);
    } else if instruction & MEM16_GROUP_MASK == MEM16_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM16_OPCODE_MASK);
        // Operand 1: 5 bit memory address at offset 0; lsb should be ignored (word aligned address)
        ir_builder.mem_address((instruction & 0b11110) as u8);
    } else if instruction & RET_CONST_GROUP_MASK == RET_CONST_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & RET_CONST_OPCODE_MASK);
        // Operand 1: 8 bit immediate at offset 0
        ir_builder.immediate((instruction & 0b11111111) as u8);
    } else if instruction & MEM_BIT_OPS_GROUP_MASK == MEM_BIT_OPS_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM_BIT_OPS_OPCODE_MASK);
        // Operand 1: 4 bit memory address at offset 0
        ir_builder.mem_address((instruction & 0b1111) as u8);
        // Operand 2: 3 bit offset of bit at offset 5
        // Convention: always place bit pos to u16 ir operand
        ir_builder.bit_index(((instruction >> 5) & 0b111) as u8);
    } else if instruction & MEM_AND_ACC_GROUP_MASK == MEM_AND_ACC_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM_AND_ACC_OPCODE_MASK);
        // Operand 1: 6 bit memory address at offset 0
        ir_builder.mem_address((instruction & 0b111111) as u8);
    } else if instruction & MEM_GROUP_MASK == MEM_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM_OPCODE_MASK);
        // Operand 1: 6 bit memory address at offset 0
        ir_builder.mem_address((instruction & 0b111111) as u8);
    } else if instruction & IO_BIT_OPS_GROUP_MASK == IO_BIT_OPS_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & IO_BIT_OPS_OPCODE_MASK);
        // Operand 1: 5 bit memory address at offset 0
        ir_builder.mem_address((instruction & 0b11111) as u8);
        // Operand 2: 3 bit pos of bit at offset 5
        // Convention: always place bit pos to u16 ir operand
        ir_builder.bit_index(((instruction >> 5) & 0b111) as u8);
    } else if instruction & ACC_CONST_GROUP_MASK == ACC_CONST_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & ACC_CONST_OPCODE_MASK);
        // Operand 1: 8 bit immediate at offset 0
        // Convention: always place immediate to u16 ir operand
        ir_builder.immediate((instruction & 0b11111111) as u8);
    } else if instruction & JUMP_GROUP_MASK == JUMP_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & JUMP_OPCODE_MASK);
        // Operand 1: 10 bit immediate at offset 0
        ir_builder.rom_address(instruction & 0b1111111111);
    } else {
        opcode_stamp = OpcodeStamp::Nop;
    }

    ir_builder.ir_opcode(opcode_stamp.to_ir_opcode());
    Ok(ir_builder.build())
}
