use super::{
    Byte,
    Word,
    pdk_core::{
        ARITH_FLAGS_MASK,
        FLAG_ZERO_MASK,
        FLAG_AUX_CARRY_MASK,
        FLAG_CARRY_MASK,
        FLAG_CARRY_OFFSET,
        FLAG_OVERFLOW_MASK,
    }
};


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

#[inline(always)]
pub fn add(acc: Byte, value: Byte, old_flags: Byte) -> (Byte, Byte) {
    add_impl(acc, value, old_flags, 0)
}

#[inline(always)]
pub fn addc(acc: Byte, value: Byte, old_flags: Byte) -> (Byte, Byte) {
    let carry = (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET;
    add_impl(acc, value, old_flags, carry)
}

#[inline(always)]
pub fn sub(acc: Byte, value: Byte, old_flags: Byte) -> (Byte, Byte) {
    sub_impl(acc, value, old_flags, 0)
}

#[inline(always)]
pub fn subc(acc: Byte, value: Byte, old_flags: Byte) -> (Byte, Byte) {
    let carry = (old_flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET;
    sub_impl(acc, value, old_flags, carry)
}

pub fn and(mut acc: Byte, value: Byte, mut flags: Byte)  -> (Byte, Byte) {
    acc &= value;
    if acc == 0 {
        flags |= FLAG_ZERO_MASK
    } else {
        flags &= !FLAG_ZERO_MASK
    }
    (acc, flags)
}

pub fn or(mut acc: Byte, value: Byte, mut flags: Byte)  -> (Byte, Byte) {
    acc |= value;
    if acc == 0 {
        flags |= FLAG_ZERO_MASK
    } else {
        flags &= !FLAG_ZERO_MASK
    }
    (acc, flags)
}

pub fn xor(mut acc: Byte, value: Byte, mut flags: Byte)  -> (Byte, Byte) {
    acc ^= value;
    if acc == 0 {
        flags |= FLAG_ZERO_MASK
    } else {
        flags &= !FLAG_ZERO_MASK
    }
    (acc, flags)
}

pub fn not(mut acc: Byte, mut flags: Byte) -> (Byte, Byte) {
    acc = !acc;
    if acc == 0 {
        flags |= FLAG_ZERO_MASK
    } else {
        flags &= !FLAG_ZERO_MASK
    }
    (acc, flags)
}

pub fn neg(mut acc: Byte, mut flags: Byte) -> (Byte, Byte) {
    acc = (!acc).wrapping_add(1);
    if acc == 0 {
        flags |= FLAG_ZERO_MASK
    } else {
        flags &= !FLAG_ZERO_MASK
    }
    (acc, flags)
}

pub fn sr(acc: Byte, mut flags: Byte) -> (Byte, Byte) {
    if acc & 0x01 != 0 {
        flags |= FLAG_CARRY_MASK
    } else {
        flags &= !FLAG_CARRY_MASK
    }
    (acc >> 1, flags)
}

pub fn sl(acc: Byte, mut flags: Byte) -> (Byte, Byte) {
    if acc & 0x80 != 0 {
        flags |= FLAG_CARRY_MASK
    } else {
        flags &= !FLAG_CARRY_MASK
    }
    (acc << 1, flags)
}

pub fn src(acc: Byte, mut flags: Byte) -> (Byte, Byte) {
    let head = ((flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET) << 7;
    if acc & 0x01 != 0 {
        flags |= FLAG_CARRY_MASK
    } else {
        flags &= !FLAG_CARRY_MASK
    }
    ((acc >> 1) | head, flags)
}

pub fn slc(acc: Byte, mut flags: Byte) -> (Byte, Byte) {
    let tail = (flags & FLAG_CARRY_MASK) >> FLAG_CARRY_OFFSET;
    if acc & 0x80 != 0 {
        flags |= FLAG_CARRY_MASK
    } else {
        flags &= !FLAG_CARRY_MASK
    }
    ((acc << 1) | tail, flags)
}

#[inline(always)]
fn add_impl(acc: Byte, value: Byte, old_flags: Byte, carry: Byte) -> (Byte, Byte) {
    let mut flags = old_flags & !ARITH_FLAGS_MASK;
    let result = (acc as Word).wrapping_add(value as Word).wrapping_add(carry as Word);
    let result8 = result as Byte;
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
fn sub_impl(acc: Byte, value: Byte, old_flags: Byte, carry: Byte) -> (Byte, Byte) {
    let mut flags = old_flags & !ARITH_FLAGS_MASK;
    let result = (acc as Word).wrapping_sub(value as Word).wrapping_add(carry as Word);
    let result8 = result as Byte;
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

