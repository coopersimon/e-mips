// Common helper ops.

// BITWISE

/// Make a value with the selected bit set to 1.
pub const fn bit(num: usize) -> u32 {
    1 << num
}

/// Test if a bit in `val` is set to 1.
pub const fn test_bit(val: u32, bit_num: usize) -> bool {
    let bit = bit(bit_num);
    (val & bit) != 0
}

// BYTEWISE

/// Make a 16-bit value from two 8-bit values.
pub const fn make16(lo: u8, hi: u8) -> u16 {
    (lo as u16) | ((hi as u16) << 8)
}

/// Get the low byte of a 16-bit value.
pub const fn lo16(val: u16) -> u8 {
    val as u8
}

/// Get the high byte of a 16-bit value.
pub const fn hi16(val: u16) -> u8 {
    (val >> 8) as u8
}

/// Make a 32-bit value from four 8-bit values.
/// The bytes are in order from least significant to most significant.
pub const fn make32(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    (b0 as u32) | ((b1 as u32) << 8) | ((b2 as u32) << 16) | ((b3 as u32) << 24)
}

/// Get the bytes from a 32-bit value.
/// The bytes are in order from least significant to most significant.
pub const fn bytes32(val: u32) -> (u8, u8, u8, u8) {
    (val as u8, (val >> 8) as u8, (val >> 16) as u8, (val >> 24) as u8)
}

/// Make a 64-bit value from eight 8-bit values.
pub const fn make64(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8, b7: u8) -> u64 {
    (b0 as u64) | ((b1 as u64) << 8) | ((b2 as u64) << 16) | ((b3 as u64) << 24) |
    ((b4 as u64) << 32) | ((b5 as u64) << 40) | ((b6 as u64) << 48) | ((b7 as u64) << 56)
}

/// Get the bytes from a 64-bit value.
pub const fn bytes64(val: u64) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (val as u8, (val >> 8) as u8, (val >> 16) as u8, (val >> 24) as u8,
    (val >> 32) as u8, (val >> 40) as u8, (val >> 48) as u8, (val >> 56) as u8)
}

/// Get the low word of a 64-bit value.
pub const fn lo64(val: u64) -> u32 {
    val as u32
}

/// Get the high word of a 64-bit value.
pub const fn hi64(val: u64) -> u32 {
    (val >> 32) as u32
}

/// Sign-extend a 8-bit value.
/// Returns a 32-bit unsigned value.
pub const fn sign_extend_8(val: u8) -> u32 {
    ((val as i8) as i32) as u32
}

/// Sign-extend a 16-bit value.
/// Returns a 32-bit unsigned value.
pub const fn sign_extend_16(val: u16) -> u32 {
    ((val as i16) as i32) as u32
}

/// Sign-extend a 32-bit value.
/// Returns a 64-bit signed value.
pub const fn sign_extend_32(val: u32) -> i64 {
    (val as i32) as i64
}