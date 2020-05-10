use crate::isa::pdk13::{
    bus::{Bus, BusExt},
    pdk_core::PdkCore,
};

use super::mock_bus::MockBus;
use crate::isa::pdk13::ir::{IrOpcode, IrSlotBuilder};

#[test]
fn initial_state_valid() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    assert_eq!(0x00, core.acc());
    assert_eq!(0x000, core.pc());
}

#[test]
fn nop_is_valid() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nop).build();
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(0x001, core.pc());
}

#[test]
fn ldsptl_is_valid() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    // Adjust sp (Pointer to ROM word address)
    bus.write_sp(0x1A);
    // Adjust ram (ROM word address)
    bus.ram[0x1A] = 0xBA;
    bus.ram[0x1B] = 0x02;
    // Adjust rom (Actual rom data)
    bus.rom[0x02BA] = IrSlotBuilder::new().original_word(0x34).build();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Ldsptl).build();
    core.step(&mut bus);
    assert_eq!(0x34, core.acc())
}

#[test]
fn ldspth_is_valid() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    // Adjust sp (Pointer to ROM word address)
    bus.write_sp(0x1A);
    // Adjust ram (ROM word address)
    bus.ram[0x1A] = 0xBA;
    bus.ram[0x1B] = 0x02;
    // Adjust rom (Actual rom data)
    bus.rom[0x02BA] = IrSlotBuilder::new().original_word(0x1234).build();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Ldspth).build();
    core.step(&mut bus);
    assert_eq!(0x12, core.acc())
}

#[test]
fn movak_sets_acc_value() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    core.step(&mut bus);
    assert_eq!(0x42, core.acc());
}

#[test]
fn addca_without_carry_is_nop_with_flags() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x42, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addca_with_carry_is_valid_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x4F)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x50, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addca_with_carry_is_valid_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x7F)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x80, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn addca_with_carry_is_valid_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Addca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subca_without_carry_is_nop_with_flags() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x42, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x20)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x1F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x80)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x7F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0xFF, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subca_with_carry_is_valid_4() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Subca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsna_skips_instruction_on_if_acc_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x03, core.pc());
    core.step(&mut bus); // 2T instruction check
    assert_eq!(0x00, core.acc());
    assert_eq!(0x03, core.pc());
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsna_not_skips_instruction_on_if_acc_non_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFE)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0xFF, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsna_has_valid_flags_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x1F)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x20, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsna_has_valid_flags_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x7F)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Izsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x80, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn dzsna_skips_instruction_on_if_acc_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x03, core.pc());
    core.step(&mut bus); // 2T instruction check
    assert_eq!(0x00, core.acc());
    assert_eq!(0x03, core.pc());
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn dzsna_not_skips_instruction_on_if_acc_non_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x02)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x01, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn dzsna_has_valid_flags_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x20)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x1F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn dzsna_has_valid_flags_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x80)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Dzsna).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x7F, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn pcadda_adds_pc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x20)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Pcadda).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x21, core.pc());
    core.step(&mut bus);
    assert_eq!(0x20, core.acc());
    assert_eq!(0x21, core.pc());
}

#[test]
fn nota_inverts_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x3C)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nota).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0xC3, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn nota_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nota).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn nega_negates_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x0F)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nega).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0xF1, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn nega_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Nega).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn sra_shifts_right() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sra).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn sra_sets_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x13)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sra).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn sla_shifts_left() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sla).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x24, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn sla_sets_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x81)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Sla).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x02, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn srca_shifts_right() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn srca_sets_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x13)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x09, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn srca_uses_carry_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x89, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn srca_uses_carry_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x13)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Srca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x89, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn slca_shifts_left() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x24, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn slca_sets_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x81)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x02, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn slca_uses_carry_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x21)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x43, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn slca_uses_carry_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xC1)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Slca).build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x83, core.acc());
    assert_eq!(0x02, core.pc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn swapa_swaps_acc_nibbles() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xC4)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Swapa).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x4C, core.acc());
    assert_eq!(0x02, core.pc());
}

#[test]
fn wdtreset_triggers_wdt_reset() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Wdreset).build();
    core.step(&mut bus);
    assert_eq!(true, bus.wdt_reset_active);
}

#[test]
fn pushaf_pushes_acc_and_flags() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.write_sp(0x10);
    bus.write_flags(0x34);
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x12)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Pushaf).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x12, bus.read_ram(0x10));
    assert_eq!(0x34, bus.read_ram(0x11));
    assert_eq!(0x12, bus.read_sp());
}

#[test]
fn popaf_pops_acc_and_flags() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.write_sp(0x12);
    bus.write_ram(0x10, 0x12);
    bus.write_ram(0x11, 0x34);
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Popaf).build();
    core.step(&mut bus);
    assert_eq!(0x12, core.acc());
    assert_eq!(0x34, bus.read_flags());
    assert_eq!(0x10, bus.read_sp());
}

#[test]
fn reset_resets_device() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Reset).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.reset_active);
    assert_eq!(0, core.acc());
    assert_eq!(0, core.pc());
    assert_eq!(false, core.global_interrupts_enabled())
}

#[test]
fn stopsys_triggers_stop_sys_signal() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Stopsys).build();
    core.step(&mut bus);
    assert_eq!(true, bus.stop_sys_active);
}

#[test]
fn stopexe_triggers_stop_exe_signal() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Stopexe).build();
    core.step(&mut bus);
    assert_eq!(true, bus.stop_exe_active);
}

#[test]
fn engint_enables_global_interrupts() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Engint).build();
    core.step(&mut bus);
    assert_eq!(true, core.global_interrupts_enabled());
}

#[test]
fn disgint_disables_global_interrupts() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Engint).build();
    bus.rom[1] = IrSlotBuilder::new().ir_opcode(IrOpcode::Disgint).build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(false, core.global_interrupts_enabled());
}

#[test]
fn ret_performs_return() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.write_sp(0x04);
    bus.write_ram_word(0x02, 0x123);
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Ret).build();
    core.step(&mut bus);
    assert_eq!(0x123, core.pc());
    core.step(&mut bus);
    assert_eq!(0x123, core.pc());
    assert_eq!(0x02, bus.read_sp());
}

#[test]
fn reti_performs_return_from_interrupt() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.write_sp(0x04);
    bus.write_ram_word(0x02, 0x123);
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Reti).build();
    core.step(&mut bus);
    assert_eq!(0x123, core.pc());
    core.step(&mut bus);
    assert_eq!(0x123, core.pc());
    assert_eq!(0x02, bus.read_sp());
    assert_eq!(true, core.global_interrupts_enabled());
}

#[test]
fn mul_is_nop() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new().ir_opcode(IrOpcode::Mul).build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    assert_eq!(0x00, core.acc());
}

#[test]
fn xorioa_xores_io_with_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0b10101010;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b11110000)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xorioa)
        .io_address(0x10)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b01011010, bus.read_io(0x10));
}

#[test]
fn movioa_moves_acc_to_io() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movioa)
        .mem_address(0x10)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x42, bus.read_io(0x10));
}

#[test]
fn movaio_moves_io_to_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0x42;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movaio)
        .io_address(0x10)
        .build();
    core.step(&mut bus);
    assert_eq!(0x42, core.acc());
}

#[test]
fn movaio_sets_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movaio)
        .io_address(0x10)
        .build();
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn stt16_loads_tim16_from_ram() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0x34;
    bus.ram[0x11] = 0x12;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Stt16)
        .mem_address(0x10)
        .build();
    core.step(&mut bus);
    assert_eq!(0x1234, bus.tim16);
}

#[test]
fn ldt16_saves_tim16_to_ram() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.tim16 = 0x1234;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ldt16)
        .mem_address(0x10)
        .build();
    core.step(&mut bus);
    assert_eq!(0x34, bus.read_ram(0x10));
    assert_eq!(0x12, bus.read_ram(0x11));
}

#[test]
fn idxmma_performs_indirect_store() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x02] = 0x24;
    bus.ram[0x03] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Idxmma)
        .mem_address(0x02)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x42, bus.ram[0x24]);
}

#[test]
fn idxmam_performs_indirect_load() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x02] = 0x24;
    bus.ram[0x03] = 0x00;
    bus.ram[0x24] = 0x42;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Idxmam)
        .mem_address(0x02)
        .build();
    core.step(&mut bus);

    assert_eq!(0x42, core.acc());
}

#[test]
fn retk_performs_ret_with_immediate() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.write_sp(0x04);
    bus.write_ram_word(0x02, 0x123);
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Retk)
        .immediate(0x42)
        .build();
    core.step(&mut bus);
    assert_eq!(0x123, core.pc());
    core.step(&mut bus);
    assert_eq!(0x123, core.pc());
    assert_eq!(0x02, bus.read_sp());
    assert_eq!(0x42, core.acc());
}

#[test]
fn t0snm_skips_instruction_if_bit_clear_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0xEF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snm)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snm_skips_instruction_if_bit_clear_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snm)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snm_skips_instruction_if_bit_clear_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snm)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snm_not_skips_instruction_if_bit_set_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snm)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snm_not_skips_instruction_if_bit_set_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snm)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snm_not_skips_instruction_if_bit_set_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snm)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snm_skips_instruction_if_bit_set_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snm)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snm_skips_instruction_if_bit_set_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snm)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snm_skips_instruction_if_bit_set_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snm)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snm_not_skips_instruction_if_bit_clear_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0xEF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snm)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snm_not_skips_instruction_if_bit_clear_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snm)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snm_not_skips_instruction_if_bit_clear_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x0E] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snm)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snio_skips_instruction_if_bit_clear_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0xEF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snio)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snio_skips_instruction_if_bit_clear_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snio)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snio_skips_instruction_if_bit_clear_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snio)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snio_not_skips_instruction_if_bit_set_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snio)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snio_not_skips_instruction_if_bit_set_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snio)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t0snio_not_skips_instruction_if_bit_set_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T0snio)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snio_skips_instruction_if_bit_set_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snio)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snio_skips_instruction_if_bit_set_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snio)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snio_skips_instruction_if_bit_set_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snio)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snio_not_skips_instruction_if_bit_clear_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0xEF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snio)
        .mem_address(0x0E)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snio_not_skips_instruction_if_bit_clear_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snio)
        .mem_address(0x0E)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn t1snio_not_skips_instruction_if_bit_clear_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x0E] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::T1snio)
        .mem_address(0x0E)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
}

#[test]
fn set0m_clears_bit_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set0m)
        .mem_address(0x10)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0xEF, bus.ram[0x10]);
}

#[test]
fn set0m_clears_bit_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set0m)
        .mem_address(0x10)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0xFE, bus.ram[0x10]);
}

#[test]
fn set0m_clears_bit_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set0m)
        .mem_address(0x10)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x7F, bus.ram[0x10]);
}

#[test]
fn set1m_sets_bit_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set1m)
        .mem_address(0x10)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x10, bus.ram[0x10]);
}

#[test]
fn set1m_sets_bit_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set1m)
        .mem_address(0x10)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, bus.ram[0x10]);
}

#[test]
fn set1m_sets_bit_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x10] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set1m)
        .mem_address(0x10)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x80, bus.ram[0x10]);
}

#[test]
fn set0io_clears_bit_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set0io)
        .mem_address(0x10)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0xEF, bus.io[0x10]);
}

#[test]
fn set0io_clears_bit_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set0io)
        .mem_address(0x10)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0xFE, bus.io[0x10]);
}

#[test]
fn set0io_clears_bit_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set0io)
        .mem_address(0x10)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x7F, bus.io[0x10]);
}

#[test]
fn set1io_sets_bit_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set1io)
        .mem_address(0x10)
        .bit_index(4)
        .build();
    core.step(&mut bus);
    assert_eq!(0x10, bus.io[0x10]);
}

#[test]
fn set1io_sets_bit_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set1io)
        .mem_address(0x10)
        .bit_index(0)
        .build();
    core.step(&mut bus);
    assert_eq!(0x01, bus.io[0x10]);
}

#[test]
fn set1io_sets_bit_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.io[0x10] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Set1io)
        .mem_address(0x10)
        .bit_index(7)
        .build();
    core.step(&mut bus);
    assert_eq!(0x80, bus.io[0x10]);
}

#[test]
fn goto_sets_pc_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Goto)
        .rom_address(0x1FF)
        .build();
    core.step(&mut bus);
    assert_eq!(0x1FF, core.pc());
    core.step(&mut bus);
    assert_eq!(0x1FF, core.pc());
}

#[test]
fn goto_sets_pc_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Goto)
        .rom_address(0x000)
        .build();
    core.step(&mut bus);
    assert_eq!(0x000, core.pc());
    core.step(&mut bus);
    assert_eq!(0x000, core.pc());
}

#[test]
fn goto_sets_pc_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Goto)
        .rom_address(0x7FF)
        .build();
    core.step(&mut bus);
    assert_eq!(0x7FF, core.pc());
    core.step(&mut bus);
    assert_eq!(0x7FF, core.pc());
}

#[test]
fn call_calls_function() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.write_sp(0x10);
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Goto)
        .rom_address(0x1FF)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x1FF, core.pc());
    bus.rom[0x1FF] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Call)
        .rom_address(0x6FF)
        .build();
    core.step(&mut bus);
    assert_eq!(0x6FF, core.pc());
    core.step(&mut bus);
    assert_eq!(0x6FF, core.pc());
    assert_eq!(0x12, bus.read_sp());
    assert_eq!(0x200, bus.read_ram_word(0x10));
}

#[test]
fn clearm_clears_memory_cell() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Clearm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
}

#[test]
fn xchm_exchanges_memory_with_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x42;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .mem_address(0x21)
        .immediate(0x21)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xchm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x21, bus.ram[0x21]);
    assert_eq!(0x42, core.acc());
}

#[test]
fn notm_inverts_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xF0;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Notm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x0F, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn notm_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Notm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn negm_negates_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x0F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Negm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0xF1, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn negm_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Negm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn srm_shifts_right() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Srm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x08, bus.ram[0x21]);
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn srm_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Srm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x08, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn slm_shifts_left() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Slm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x22, bus.ram[0x21]);
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn slm_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x81;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Slm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn srcm_shifts_right() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Srcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x08, bus.ram[0x21]);
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn srcm_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Srcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x08, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn srcm_uses_carry_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Srcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x08, bus.ram[0x21]);
}

#[test]
fn srcm_uses_carry_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Srcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x88, bus.ram[0x21]);
}

#[test]
fn slcm_shifts_left() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Slcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x22, bus.ram[0x21]);
    assert_eq!(false, bus.is_carry_flag());
}

#[test]
fn slcm_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x81;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Slcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x02, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn slcm_uses_carry_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Slcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x22, bus.ram[0x21]);
}

#[test]
fn slcm_uses_carry_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Slcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x23, bus.ram[0x21]);
}

#[test]
fn movma_moves_acc_to_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movma)
        .immediate(0x22)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x42, bus.ram[0x22]);
}

#[test]
fn andak_ands_acc_with_immediate() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Andak)
        .immediate(0b10011001)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b10001000, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn andak_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Andak)
        .immediate(0x00)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn orak_ors_acc_with_immediate() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Orak)
        .immediate(0b10011001)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b10111011, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn orak_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Orak)
        .immediate(0x00)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn xorak_xors_acc_with_immediate() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xorak)
        .immediate(0b10011001)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b00110011, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn xorak_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xorak)
        .immediate(0x00)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn movam_moves_mem_to_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x42;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x42, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn movam_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn andam_ands_acc_with_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0b10011001;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Andam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b10001000, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn andam_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Andam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn oram_ors_acc_with_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0b10011001;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Oram)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b10111011, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn oram_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Oram)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn xoram_xors_acc_with_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0b10011001;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xoram)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b00110011, core.acc());
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn xoram_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xoram)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn andma_ands_acc_with_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0b10011001;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Andma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b10001000, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn andma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0xFF)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Andma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn orma_ors_acc_with_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0b10011001;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Orma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b10111011, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn orma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Orma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn xorma_xors_acc_with_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0b10011001;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0b10101010)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xorma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0b00110011, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
}

#[test]
fn xorma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x00)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Xorma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn addak_adds_immediate_to_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addak)
        .immediate(4)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(7, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addak_sets_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(255)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addak)
        .immediate(1)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn addak_sets_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(255)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addak)
        .immediate(16)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(15, core.acc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn addak_sets_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addak)
        .immediate(5)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(20, core.acc());
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn addak_sets_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(127)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addak)
        .immediate(5)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(132, core.acc());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subak_subs_immediate_from_acc() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(8)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subak)
        .immediate(7)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(1, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subak_sets_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subak)
        .immediate(15)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn subak_sets_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subak)
        .immediate(3)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(254, core.acc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn subak_sets_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(17)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subak)
        .immediate(3)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(14, core.acc());
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn subak_sets_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(128)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subak)
        .immediate(5)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(123, core.acc());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn ceqsnak_skips_instruction_on_if_acc_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnak)
        .immediate(0x42)
        .build();
    core.step(&mut bus);

    core.step(&mut bus);
    assert_eq!(0x03, core.pc());
    core.step(&mut bus);
    assert_eq!(0x03, core.pc());

    assert_eq!(0x42, core.acc());
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn ceqsnak_not_skips_instruction_on_if_acc_non_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnak)
        .immediate(0x41)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    assert_eq!(0x42, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn ceqsnak_sets_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnak)
        .immediate(15)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn ceqsnak_sets_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnak)
        .immediate(3)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn ceqsnak_sets_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(17)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnak)
        .immediate(3)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn ceqsnak_sets_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(128)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnak)
        .immediate(5)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn ceqsnam_skips_instruction_on_if_mem_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x42;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    core.step(&mut bus);
    assert_eq!(0x03, core.pc());
    core.step(&mut bus);
    assert_eq!(0x03, core.pc());

    assert_eq!(0x42, core.acc());
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn ceqsnam_not_skips_instruction_on_if_mem_non_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x41;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x42)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    assert_eq!(0x42, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn ceqsnam_sets_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 15;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn ceqsnam_sets_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn ceqsnam_sets_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(17)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn ceqsnam_sets_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 5;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(128)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Ceqsnam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn addcm_without_carry_is_nop_with_flags() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x42;
    bus.rom[0] = IrSlotBuilder::new()
        .mem_address(0x21)
        .ir_opcode(IrOpcode::Addcm)
        .build();
    core.step(&mut bus);
    assert_eq!(0x42, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addcm_with_carry_is_valid_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x4F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x50, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addcm_with_carry_is_valid_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x80, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn addcm_with_carry_is_valid_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcm_without_carry_is_nop_with_flags() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x42;
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    assert_eq!(0x42, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcm_with_carry_is_valid_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x20;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x1F, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcm_with_carry_is_valid_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x7F, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subcm_with_carry_is_valid_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0xFF, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcm_with_carry_is_valid_4() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsnm_skips_instruction_on_if_acc_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Izsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsnm_not_skips_instruction_on_if_acc_non_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Izsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x01, core.pc());
    assert_eq!(0xFF, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsnm_has_valid_flags_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x1F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Izsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x20, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn izsnm_has_valid_flags_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Izsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x80, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn dzsnm_skips_instruction_on_if_acc_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Dzsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());
    core.step(&mut bus);
    assert_eq!(0x02, core.pc());

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn dzsnm_not_skips_instruction_on_if_acc_non_zero() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x02;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Dzsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);

    assert_eq!(0x01, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn dzsnm_has_valid_flags_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x20;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Dzsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);

    assert_eq!(0x1F, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn dzsnm_has_valid_flags_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Dzsnm)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);

    assert_eq!(0x7F, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn incm_increments() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x20;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Incm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0x21, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn incm_has_valid_flags_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Incm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn incm_has_valid_flags_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x7F;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Incm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0x80, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn decm_decrements() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x19;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Decm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0x18, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn decm_has_valid_flags_1() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x01;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Decm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn decm_has_valid_flags_2() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x80;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Decm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0x7F, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn decm_has_valid_flags_3() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x00;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Decm)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);

    assert_eq!(0xFF, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(true, bus.is_carry_flag());
    assert_eq!(true, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addma_adds_acc_to_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x03;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x04)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x07, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn addma_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x03)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x01, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn addma_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(25, bus.ram[0x21]);
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn addma_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 127;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(128, bus.ram[0x21]);
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subma_subs_acc_from_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(8, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 1;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn subma_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 2;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(255, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn subma_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 17;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(14, bus.ram[0x21]);
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn subma_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 128;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(127, bus.ram[0x21]);
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn addcma_adds_acc_to_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x03;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x04)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x07, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addcma_uses_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x03;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x04)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcma)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x08, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addcma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn addcma_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x03)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x01, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn addcma_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(25, bus.ram[0x21]);
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn addcma_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 127;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(128, bus.ram[0x21]);
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subcma_subs_acc_from_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(8, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcma_uses_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 11;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcma)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(7, bus.ram[0x21]);
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcma_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 1;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, bus.ram[0x21]);
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn subcma_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 2;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(255, bus.ram[0x21]);
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn subcma_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 17;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(14, bus.ram[0x21]);
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn subcma_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 128;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcma)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(127, bus.ram[0x21]);
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn addam_adds_acc_to_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0x03;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x04)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x07, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addam_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x01)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn addam_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x03)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x01, core.acc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn addam_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(25, core.acc());
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn addam_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 127;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(128, core.acc());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subam_subs_acc_from_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(11)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(8, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subam_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 1;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn subam_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(2)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(255, core.acc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn subam_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(17)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(14, core.acc());
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn subam_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 1;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(128)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(127, core.acc());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn addcam_adds_acc_to_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 4;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(3)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(7, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addcam_uses_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(4)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcam)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(8, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn addcam_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFF;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn addcam_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 0xFE;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(0x03)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x01, core.acc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn addcam_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 10;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(15)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(25, core.acc());
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn addcam_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 127;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Addcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(128, core.acc());
    assert_eq!(true, bus.is_overflow_flag());
}

#[test]
fn subcam_subs_acc_from_mem() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(11)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(8, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcam_uses_carry() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(11)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcam)
        .mem_address(0x21)
        .build();
    bus.set_carry_flag(true);
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(7, core.acc());
    assert_eq!(false, bus.is_zero_flag());
    assert_eq!(false, bus.is_carry_flag());
    assert_eq!(false, bus.is_aux_carry_flag());
    assert_eq!(false, bus.is_overflow_flag());
}

#[test]
fn subcam_changes_zero_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 1;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(1)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(0x00, core.acc());
    assert_eq!(true, bus.is_zero_flag());
}

#[test]
fn subcam_changes_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(2)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(255, core.acc());
    assert_eq!(true, bus.is_carry_flag());
}

#[test]
fn subcam_changes_aux_carry_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 3;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(17)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(14, core.acc());
    assert_eq!(true, bus.is_aux_carry_flag());
}

#[test]
fn subcam_changes_overflow_flag() {
    let mut bus = MockBus::new();
    let mut core = PdkCore::new();
    bus.ram[0x21] = 1;
    bus.rom[0] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Movak)
        .immediate(128)
        .build();
    bus.rom[1] = IrSlotBuilder::new()
        .ir_opcode(IrOpcode::Subcam)
        .mem_address(0x21)
        .build();
    core.step(&mut bus);
    core.step(&mut bus);

    assert_eq!(127, core.acc());
    assert_eq!(true, bus.is_overflow_flag());
}
