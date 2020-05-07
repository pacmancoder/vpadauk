use crate::isa::pdk13::{
    bus::{Bus, BusExt},
    pdk_core::PdkCore,
};

use super::mock_bus::MockBus;
use crate::isa::pdk13::ir::{IrOpcode, IrSlotBuilder};


#[test]
fn initial_state_valid() {
    let core = PdkCore::new(MockBus::new());
    assert_eq!(0x00, core.acc());
    assert_eq!(0x000, core.pc());
}

#[test]
fn nop_is_valid() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nop).build();
    core.step();
    assert_eq!(0x00, core.acc());
    assert_eq!(0x001, core.pc());
}

#[test]
fn ldsptl_is_valid() {
    let mut core = PdkCore::new(MockBus::new());
    // Adjust sp (Pointer to ROM word address)
    core.bus().write_sp(0x1A);
    // Adjust ram (ROM word address)
    core.bus().ram[0x1A] = 0xBA;
    core.bus().ram[0x1B] = 0x02;
    // Adjust rom (Actual rom data)
    core.bus().rom[0x02BA] = IrSlotBuilder::new().original_word(0x34).build();
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Ldsptl).build();
    core.step();
    assert_eq!(0x34, core.acc())
}

#[test]
fn ldspth_is_valid() {
    let mut core = PdkCore::new(MockBus::new());
    // Adjust sp (Pointer to ROM word address)
    core.bus().write_sp(0x1A);
    // Adjust ram (ROM word address)
    core.bus().ram[0x1A] = 0xBA;
    core.bus().ram[0x1B] = 0x02;
    // Adjust rom (Actual rom data)
    core.bus().rom[0x02BA] = IrSlotBuilder::new().original_word(0x1234).build();
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Ldspth).build();
    core.step();
    assert_eq!(0x12, core.acc())
}

#[test]
fn movak_sets_acc_value() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    core.step();
    assert_eq!(0x42, core.acc());
}

#[test]
fn addca_without_carry_is_nop_with_flags() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    core.step();
    core.step();
    assert_eq!(0x42, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(false, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn addca_with_carry_is_valid_1() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x4F)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x50, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn addca_with_carry_is_valid_2() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x7F)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x80, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(true, core.bus().is_overflow_flag());
}

#[test]
fn addca_with_carry_is_valid_3() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_zero_flag());
    assert_eq!(true, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn subca_without_carry_is_nop_with_flags() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    core.step();
    core.step();
    assert_eq!(0x42, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(false, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_1() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x20)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x1F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_2() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x80)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x7F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(true, core.bus().is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_3() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0xFF, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(true, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_4() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(false, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn izsna_skips_instruction_on_if_acc_zero() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x03, core.pc());
    core.step(); // 2T instruction check
    assert_eq!(0x00, core.acc());
    assert_eq!(0x03, core.pc());
    assert_eq!(true, core.bus().is_zero_flag());
    assert_eq!(true, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn izsna_not_skips_instruction_on_if_acc_non_zero() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFE)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0xFF, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(false, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn izsna_has_valid_flags_1() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x1F)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x20, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn izsna_has_valid_flags_2() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x7F)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x80, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(true, core.bus().is_overflow_flag());
}

#[test]
fn dzsna_skips_instruction_on_if_acc_zero() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x03, core.pc());
    core.step(); // 2T instruction check
    assert_eq!(0x00, core.acc());
    assert_eq!(0x03, core.pc());
    assert_eq!(true, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(false, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn dzsna_not_skips_instruction_on_if_acc_non_zero() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x02)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x01, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(false, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn dzsna_has_valid_flags_1() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x20)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x1F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(false, core.bus().is_overflow_flag());
}

#[test]
fn dzsna_has_valid_flags_2() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x80)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x7F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
    assert_eq!(false, core.bus().is_carry_flag());
    assert_eq!(true, core.bus().is_aux_carry_flag());
    assert_eq!(true, core.bus().is_overflow_flag());
}

#[test]
fn pcadda_adds_pc() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x20)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Pcadda).build();
    core.step();
    core.step();
    assert_eq!(0x21, core.pc());
    core.step();
    assert_eq!(0x20, core.acc());
    assert_eq!(0x21, core.pc());
}

#[test]
fn nota_inverts_acc() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x3C)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nota).build();
    core.step();
    core.step();
    assert_eq!(0xC3, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
}

#[test]
fn nota_changes_zero_flag() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nota).build();
    core.step();
    core.step();
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_zero_flag());
}

#[test]
fn nega_negates_acc() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x0F)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nega).build();
    core.step();
    core.step();
    assert_eq!(0xF1, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_zero_flag());
}

#[test]
fn nega_changes_zero_flag() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nega).build();
    core.step();
    core.step();
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_zero_flag());
}

#[test]
fn sra_shifts_right() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sra).build();
    core.step();
    core.step();
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_carry_flag());
}

#[test]
fn sra_sets_carry() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x13)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sra).build();
    core.step();
    core.step();
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_carry_flag());
}

#[test]
fn sla_shifts_left() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sla).build();
    core.step();
    core.step();
    assert_eq!(0x24, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_carry_flag());
}

#[test]
fn sla_sets_carry() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x81)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sla).build();
    core.step();
    core.step();
    assert_eq!(0x02, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_carry_flag());
}

#[test]
fn srca_shifts_right() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    core.step();
    core.step();
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_carry_flag());
}

#[test]
fn srca_sets_carry() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x13)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    core.step();
    core.step();
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_carry_flag());
}

#[test]
fn srca_uses_carry_1() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x89, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_carry_flag());
}

#[test]
fn srca_uses_carry_2() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x13)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x89, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_carry_flag());
}

#[test]
fn slca_shifts_left() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    core.step();
    core.step();
    assert_eq!(0x24, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_carry_flag());
}

#[test]
fn slca_sets_carry() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x81)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    core.step();
    core.step();
    assert_eq!(0x02, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_carry_flag());
}

#[test]
fn slca_uses_carry_1() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x21)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x43, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, core.bus().is_carry_flag());
}

#[test]
fn slca_uses_carry_2() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xC1)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    core.bus().set_carry_flag(true);
    core.step();
    core.step();
    assert_eq!(0x83, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, core.bus().is_carry_flag());
}

#[test]
fn swapa_swaps_acc_nibbles() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xC4)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Swapa).build();
    core.step();
    core.step();
    assert_eq!(0x4C, core.acc());
    assert_eq!(0x02, core.pc());
}

#[test]
fn wdtreset_triggers_wdt_reset() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Wdreset).build();
    core.step();
    assert_eq!(true, core.bus().wdt_reset_active);
}

#[test]
fn pushaf_pushes_acc_and_flags() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().write_sp(0x10);
    core.bus().write_flags(0x34);
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Pushaf).build();
    core.step();
    core.step();
    assert_eq!(0x12, core.bus().read_ram(0x10));
    assert_eq!(0x34, core.bus().read_ram(0x11));
    assert_eq!(0x12, core.bus().read_sp());
}

#[test]
fn popaf_pops_acc_and_flags() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().write_sp(0x12);
    core.bus().write_ram(0x10, 0x12);
    core.bus().write_ram(0x11, 0x34);
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Popaf).build();
    core.step();
    assert_eq!(0x12, core.acc());
    assert_eq!(0x34, core.bus().read_flags());
    assert_eq!(0x10, core.bus().read_sp());
}

#[test]
fn reset_resets_device() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Reset).build();
    core.step();
    core.step();
    assert_eq!(true, core.bus().reset_active);
    assert_eq!(0, core.acc());
    assert_eq!(0, core.pc());
    assert_eq!(false, core.global_interrupts_enabled())
}

#[test]
fn stopsys_triggers_stop_sys_signal() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Stopsys).build();
    core.step();
    assert_eq!(true, core.bus().stop_sys_active);
}

#[test]
fn stopexe_triggers_stop_exe_signal() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Stopexe).build();
    core.step();
    assert_eq!(true, core.bus().stop_exe_active);
}

#[test]
fn engint_enables_global_interrupts() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Engint).build();
    core.step();
    assert_eq!(true, core.global_interrupts_enabled());
}

#[test]
fn disgint_disables_global_interrupts() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Engint).build();
    core.bus().rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Disgint).build();
    core.step();
    core.step();
    assert_eq!(false, core.global_interrupts_enabled());
}

#[test]
fn ret_performs_return() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().write_sp(0x04);
    core.bus().write_ram_word(0x02, 0x123);
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Ret).build();
    core.step();
    assert_eq!(0x123, core.pc());
    core.step();
    assert_eq!(0x123, core.pc());
    assert_eq!(0x02, core.bus().read_sp());
}

#[test]
fn reti_performs_return_from_interrupt() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().write_sp(0x04);
    core.bus().write_ram_word(0x02, 0x123);
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Reti).build();
    core.step();
    assert_eq!(0x123, core.pc());
    core.step();
    assert_eq!(0x123, core.pc());
    assert_eq!(0x02, core.bus().read_sp());
    assert_eq!(true, core.global_interrupts_enabled());
}

#[test]
fn mul_is_nop() {
    let mut core = PdkCore::new(MockBus::new());
    core.bus().rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Mul).build();
    core.step();
    assert_eq!(0x01, core.pc());
    assert_eq!(0x00, core.acc());
}
