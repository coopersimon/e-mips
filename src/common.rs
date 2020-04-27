
// Make a 16-bit value from two 8-bit values.
macro_rules! make16 {
    ($hi:expr, $lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

// Get the low byte of a 16-bit value.
macro_rules! lo16 {
    ($val:expr) => {
        $val as u8
    };
}

// Set the low byte of a 16-bit value.
macro_rules! set_lo16 {
    ($val:expr, $lo:expr) => {
        ($val & 0xFF00) | ($lo as u16)
    };
}

// Get the high byte of a 16-bit value.
macro_rules! hi16 {
    ($val:expr) => {
        ($val >> 8) as u8
    };
}

// Set the high byte of a 16-bit value.
macro_rules! set_hi16 {
    ($val:expr, $hi:expr) => {
        ($val & 0x00FF) | (($hi as u16) << 8)
    };
}

// Make a 32-bit value from four 8-bit values.
macro_rules! make32 {
    ($b3:expr, $b2:expr, $b1:expr, $b0:expr) => {
        (($b3 as u32) << 24) | (($b2 as u32) << 16) | (($b1 as u32) << 8) | ($b0 as u32)
    };
}

// Get the bytes from a 32-bit value.
macro_rules! bytes32 {
    ($val:expr) => {
        (($val >> 24) as u8, ($val >> 16) as u8, ($val >> 8) as u8, $val as u8)
    };
}

// Make a 64-bit value from eight 8-bit values.
macro_rules! make64 {
    ($b7:expr, $b6:expr, $b5:expr, $b4:expr, $b3:expr, $b2:expr, $b1:expr, $b0:expr) => {
        (($b7 as u64) << 56) | (($b6 as u64) << 48) | (($b5 as u64) << 40) | (($b4 as u64) << 32) |
        (($b3 as u64) << 24) | (($b2 as u64) << 16) | (($b1 as u64) << 8) | ($b0 as u64)
    };
}

// Get the bytes from a 64-bit value.
macro_rules! bytes64 {
    ($val:expr) => {
        (($val >> 56) as u8, ($val >> 48) as u8, ($val >> 40) as u8, ($val >> 32) as u8,
         ($val >> 24) as u8, ($val >> 16) as u8, ($val >> 8) as u8, $val as u8)
    };
}

// Get the low word of a 64-bit value.
macro_rules! lo64 {
    ($val:expr) => {
        $val as u32
    };
}

// Get the high word of a 64-bit value.
macro_rules! hi64 {
    ($val:expr) => {
        ($val >> 32) as u32
    };
}

// Sign-extend a 16-bit value.
// Returns a 32-bit unsigned value.
macro_rules! sign_extend_16 {
    ($val:expr) => {
        (($val as i16) as i32) as u32
    };
}

// Sign-extend a 32-bit value.
// Returns a 64-bit signed value.
macro_rules! sign_extend_32 {
    ($val:expr) => {
        ($val as i32) as i64
    };
}