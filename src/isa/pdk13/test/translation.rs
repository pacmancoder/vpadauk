use crate::isa::pdk13::{
    Pdk13Error,
    ir::IrOpcode,
    translation::{
        generate_ir
    },
};

#[test]
fn too_big_opcode_produces_error() {
    assert!(generate_ir(0x2000).is_err())
}

#[test]
fn invalid_opcode_produces_not() {
    let ir = generate_ir(0x003E).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Nop)
}

#[test]
fn valid_opcode_nop() {
    let ir = generate_ir(0x0000).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Nop)
}


#[test]
fn valid_misc_opcode_min() {
    let ir = generate_ir(0x0006).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Ldsptl)
}

#[test]
fn valid_misc_opcode_max() {
    let ir = generate_ir(0x003C).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Mul)
}


#[test]
fn valid_xor_io_acc_opcode() {
    let ir = generate_ir(0x007A).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Xorioa);
    assert_eq!(ir.operand8(),  0x1A)
}

#[test]
fn valid_mov_io_opcode_min() {
    let ir = generate_ir(0x009A).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Movioa);
    assert_eq!(ir.operand8(),  0x1A)
}


#[test]
fn valid_mov_io_opcode_max() {
    let ir = generate_ir(0x00BA).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Movaio);
    assert_eq!(ir.operand8(),  0x1A)
}

#[test]
fn valid_stt16m_opcode() {
    let ir = generate_ir(0x00DA).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Stt16);
    assert_eq!(ir.operand8(),  0x1A);
}

#[test]
fn valid_ldt16_opcode() {
    let ir = generate_ir(0x00DB).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Ldt16);
    assert_eq!(ir.operand8(),  0x1A);
}

#[test]
fn valid_idxmma_opcode() {
    let ir = generate_ir(0x00FA).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Idxmma);
    assert_eq!(ir.operand8(),  0x1A);
}

#[test]
fn valid_idxmam_opcode() {
    let ir = generate_ir(0x00FB).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Idxmam);
    assert_eq!(ir.operand8(),  0x1A);
}

#[test]
fn valid_retk_min_opcode() {
    let ir = generate_ir(0x0100).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Retk);
    assert_eq!(ir.operand16(),  0x00);
}

#[test]
fn valid_retk_max_opcode() {
    let ir = generate_ir(0x01FF).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Retk);
    assert_eq!(ir.operand16(),  0xFF);
}

#[test]
fn valid_t0snm_opcode() {
    let ir = generate_ir(0b10_101_0_1010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::T0snm);
    assert_eq!(ir.operand8(), 0x0A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_t1snm_opcode() {
    let ir = generate_ir(0b10_101_1_1010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::T1snm);
    assert_eq!(ir.operand8(), 0x0A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_set0m_opcode() {
    let ir = generate_ir(0b11_101_0_1010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Set0m);
    assert_eq!(ir.operand8(), 0x0A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_set1m_opcode() {
    let ir = generate_ir(0b11_101_1_1010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Set1m);
    assert_eq!(ir.operand8(), 0x0A);
    assert_eq!(ir.operand16(), 0x05);
}


#[test]
fn valid_mem_and_acc_min_opcode() {
    let ir = generate_ir(0x043A).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Addma);
    assert_eq!(ir.operand8(),  0x3A);
}

#[test]
fn valid_mem_and_acc_max_opcode() {
    let ir = generate_ir(0x07FA).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Movam);
    assert_eq!(ir.operand8(),  0x3A);
}

#[test]
fn valid_mem_min_opcode() {
    let ir = generate_ir(0x083A).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Addcm);
    assert_eq!(ir.operand8(),  0x3A);
}

#[test]
fn valid_mem_max_opcode() {
    let ir = generate_ir(0x0BBA).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Ceqsnam);
    assert_eq!(ir.operand8(),  0x3A);
}

#[test]
fn valid_t0snio_opcode() {
    let ir = generate_ir(0b1100_101_11010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::T0snio);
    assert_eq!(ir.operand8(), 0x1A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_t1snio_opcode() {
    let ir = generate_ir(0b1101_101_11010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::T1snio);
    assert_eq!(ir.operand8(), 0x1A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_set0io_opcode() {
    let ir = generate_ir(0b1110_101_11010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Set0io);
    assert_eq!(ir.operand8(), 0x1A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_set1io_opcode() {
    let ir = generate_ir(0b1111_101_11010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Set1io);
    assert_eq!(ir.operand8(), 0x1A);
    assert_eq!(ir.operand16(), 0x05);
}

#[test]
fn valid_acc_const_min_opcode() {
    let ir = generate_ir(0b10000_01011010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Addak);
    assert_eq!(ir.operand16(), 0x5A);
}

#[test]
fn valid_acc_const_max_opcode() {
    let ir = generate_ir(0b10111_01011010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Movak);
    assert_eq!(ir.operand16(), 0x5A);
}

#[test]
fn valid_goto_min_opcode() {
    let ir = generate_ir(0b110_0000000000).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Goto);
    assert_eq!(ir.operand16(), 0x000);
}

#[test]
fn valid_goto_arbitrary_opcode() {
    let ir = generate_ir(0b110_1101011010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Goto);
    assert_eq!(ir.operand16(), 0x35A);
}

#[test]
fn valid_goto_max_opcode() {
    let ir = generate_ir(0b110_1111111111).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Goto);
    assert_eq!(ir.operand16(), 0x3FF);
}

#[test]
fn valid_call_min_opcode() {
    let ir = generate_ir(0b111_0000000000).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Call);
    assert_eq!(ir.operand16(), 0x000);
}

#[test]
fn valid_call_arbitrary_opcode() {
    let ir = generate_ir(0b111_1101011010).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Call);
    assert_eq!(ir.operand16(), 0x35A);
}

#[test]
fn valid_call_max_opcode() {
    let ir = generate_ir(0b111_1111111111).unwrap();
    assert_eq!(ir.opcode(), IrOpcode::Call);
    assert_eq!(ir.operand16(), 0x3FF);
}