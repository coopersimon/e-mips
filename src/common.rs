// Common helper ops.

// BITWISE

/// Make a value with the selected bit set to 1.
#[inline(always)]
pub const fn bit(num: usize) -> u32 {
    1 << num
}

/// Test if a bit in `val` is set to 1.
#[inline(always)]
pub const fn test_bit(val: u32, bit_num: usize) -> bool {
    let bit = bit(bit_num);
    (val & bit) != 0
}

// BYTEWISE

/// Get the low word of a 64-bit value.
#[inline(always)]
pub const fn lo64(val: u64) -> u32 {
    val as u32
}

/// Get the high word of a 64-bit value.
#[inline(always)]
pub const fn hi64(val: u64) -> u32 {
    (val >> 32) as u32
}

/// Sign-extend a 8-bit value.
/// Returns a 32-bit unsigned value.
#[inline(always)]
pub const fn sign_extend_8(val: u8) -> u32 {
    ((val as i8) as i32) as u32
}

/// Sign-extend a 16-bit value.
/// Returns a 32-bit unsigned value.
#[inline(always)]
pub const fn sign_extend_16(val: u16) -> u32 {
    ((val as i16) as i32) as u32
}

/// Sign-extend a 32-bit value.
/// Returns a 64-bit signed value.
#[inline(always)]
pub const fn sign_extend_32(val: u32) -> i64 {
    (val as i32) as i64
}