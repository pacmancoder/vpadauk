use crate::isa::pdk13::ir::{IrOpcode, IrSlot, IrSlotBuilder};

#[test]
fn default_opcode_is_nop() {
    assert_eq!(IrOpcode::Nop, IrSlot::default().ir_opcode());
}

#[test]
fn separate_fields_packing_is_reversible_1() {
    let ir = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Goto)
        .original_word(0x1ADE)
        .mem_address(0xDA)
        .bit_index(0x06)
        .build();

    assert_eq!(IrOpcode::Goto, ir.ir_opcode());
    assert_eq!(0x1ADE, ir.original_word());
    assert_eq!(0xDA, ir.mem_address());
    assert_eq!(0x06, ir.bit_index());
}

#[test]
fn separate_fields_packing_is_reversible_2() {
    let ir = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Call)
        .original_word(0x1FFF)
        .mem_address(0xFF)
        .bit_index(0x07)
        .build();

    assert_eq!(IrOpcode::Call, ir.ir_opcode());
    assert_eq!(0x1FFF, ir.original_word());
    assert_eq!(0xFF, ir.mem_address());
    assert_eq!(0x07, ir.bit_index());
}

#[test]
fn separate_fields_packing_is_reversible_3() {
    let ir = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Nop)
        .original_word(0)
        .mem_address(0)
        .bit_index(0)
        .build();

    assert_eq!(IrOpcode::Nop, ir.ir_opcode());
    assert_eq!(0, ir.original_word());
    assert_eq!(0, ir.mem_address());
    assert_eq!(0, ir.bit_index());
}

#[test]
fn separate_fields_packing_is_reversible_4() {
    let ir = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Call)
        .original_word(0x123)
        .rom_address(0x6AF)
        .build();

    assert_eq!(IrOpcode::Call, ir.ir_opcode());
    assert_eq!(0x123, ir.original_word());
    assert_eq!(0x6AF, ir.rom_address());
}

#[test]
fn address_and_immediate_fields_are_same() {
    let ir = IrSlotBuilder::new().mem_address(0x42).build();
    assert_eq!(0x42, ir.io_address());
    assert_eq!(0x42, ir.immediate());
}

#[test]
fn rom_address_is_composed_from_two_fields() {
    let ir = IrSlotBuilder::new()
        .mem_address(0x42)
        .bit_index(0x6)
        .build();
    assert_eq!(0x642, ir.rom_address());
}
