use super::{
    ir::{IrSlot, IrSlotBuilder},
    limit,
    opcode_stamp::OpcodeStamp,
    Pdk13Error, Pdk13Result, Word,
};

pub fn generate_ir(instruction: Word) -> Pdk13Result<IrSlot> {
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
    let opcode_stamp;

    if instruction & MISC_GROUP_MASK == MISC_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction);
    // No operands
    } else if instruction & XOR_IO_GROUP_MASK == XOR_IO_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & XOR_IO_OPCODE_MASK);
        // Operand 1: 5 bit io address at offset 0
        ir_builder.operand8((instruction & 0b11111) as u8);
    } else if instruction & MOV_IO_GROUP_MASK == MOV_IO_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MOV_IO_OPCODE_MASK);
        // Operand 1: 5 bit io address at offset 0
        ir_builder.operand8((instruction & 0b11111) as u8);
    } else if instruction & MEM16_GROUP_MASK == MEM16_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM16_OPCODE_MASK);
        // Operand 1: 5 bit memory address at offset 0; lsb should be ignored (word aligned address)
        ir_builder.operand8((instruction & 0b11110) as u8);
    } else if instruction & RET_CONST_GROUP_MASK == RET_CONST_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & RET_CONST_OPCODE_MASK);
        // Operand 1: 8 bit immediate at offset 0
        // Convention: always place immediate to u16 ir operand
        ir_builder.operand16((instruction & 0b11111111) as u16);
    } else if instruction & MEM_BIT_OPS_GROUP_MASK == MEM_BIT_OPS_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM_BIT_OPS_OPCODE_MASK);
        // Operand 1: 4 bit memory address at offset 0
        ir_builder.operand8((instruction & 0b1111) as u8);
        // Operand 2: 3 bit offset of bit at offset 5
        // Convention: always place bit pos to u16 ir operand
        ir_builder.operand16(((instruction >> 5) & 0b111) as u16);
    } else if instruction & MEM_AND_ACC_GROUP_MASK == MEM_AND_ACC_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM_AND_ACC_OPCODE_MASK);
        // Operand 1: 6 bit memory address at offset 0
        ir_builder.operand8((instruction & 0b111111) as u8);
    } else if instruction & MEM_GROUP_MASK == MEM_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & MEM_OPCODE_MASK);
        // Operand 1: 6 bit memory address at offset 0
        ir_builder.operand8((instruction & 0b111111) as u8);
    } else if instruction & IO_BIT_OPS_GROUP_MASK == IO_BIT_OPS_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & IO_BIT_OPS_OPCODE_MASK);
        // Operand 1: 5 bit memory address at offset 0
        ir_builder.operand8((instruction & 0b11111) as u8);
        // Operand 2: 3 bit pos of bit at offset 5
        // Convention: always place bit pos to u16 ir operand
        ir_builder.operand16(((instruction >> 5) & 0b111) as u16);
    } else if instruction & ACC_CONST_GROUP_MASK == ACC_CONST_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & ACC_CONST_OPCODE_MASK);
        // Operand 1: 8 bit immediate at offset 0
        // Convention: always place immediate to u16 ir operand
        ir_builder.operand16((instruction & 0b11111111) as u16);
    } else if instruction & JUMP_GROUP_MASK == JUMP_GROUP_STAMP {
        opcode_stamp = OpcodeStamp::from_primitive(instruction & JUMP_OPCODE_MASK);
        // Operand 1: 10 bit immediate at offset 0
        ir_builder.operand16((instruction & 0b1111111111) as u16);
    } else {
        opcode_stamp = OpcodeStamp::Nop;
    }

    ir_builder.opcode(opcode_stamp.to_ir_opcode());
    Ok(ir_builder.build())
}
