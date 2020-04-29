/// Little-endian memory implementations.
#[macro_use]
mod little;

use num_traits::sign::Unsigned;

#[derive(Default, Clone, Copy)]
/// Address bus with variable width.
pub struct Addr<Width: Unsigned> {
    addr: Width
}

impl<Width: Unsigned> Addr<Width> {
    pub fn new<Other: Unsigned + Into<Width>>(val: Other) -> Self {
        Self {
            addr: val.into()
        }
    }

    pub fn inc(self) -> Self {
        Addr {
            addr: self.addr + Width::one()
        }
    }
}

/// Base memory trait.
///
/// When implementing a memory device, this trait must be implemented manually.
/// Then further memory traits can be implemented.
pub trait Memory {

    /// This type describes the width of the address bus.
    /// In MIPS this is usually the same as the data width (i.e., 32 or 64-bits.)
    type Addr: Unsigned + Copy;

    /// Read a single byte.
    fn read_byte(&mut self, addr: Self::Addr) -> u8;

    /// Write a single byte.
    fn write_byte(&mut self, addr: Self::Addr, data: u8);
}

/// Memory with a 16-bit data bus.
/// 
/// For default impls, see `impl_mem_16_little`.
pub trait Mem16: Memory {

    /// Read a 16-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_halfword(&mut self, addr: Self::Addr) -> u16;

    /// Write a 16-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_halfword(&mut self, addr: Self::Addr, data: u16);
}

/// Memory with a 32-bit data bus.
/// 
/// For default impls, see `impl_mem_32_little`.
pub trait Mem32: Mem16 {

    /// Read a 32-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_word(&mut self, addr: Self::Addr) -> u32;

    /// Write a 32-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_word(&mut self, addr: Self::Addr, data: u32);
}

/// Memory with a 64-bit data bus.
/// 
/// For default impls, see `impl_mem_64_little`.
pub trait Mem64: Mem32 {

    /// Read a 64-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_doubleword(&mut self, addr: Self::Addr) -> u64;

    /// Write a 64-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_doubleword(&mut self, addr: Self::Addr, data: u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LittleMemTest {
        bytes: Vec<u8>
    }

    impl LittleMemTest {
        fn new(size: usize) -> Self {
            Self {
                bytes: vec![0; size]
            }
        }
    }

    impl Memory for LittleMemTest {
        type Addr = u32;

        fn read_byte(&mut self, addr: Self::Addr) -> u8 {
            self.bytes[addr as usize]
        }

        fn write_byte(&mut self, addr: Self::Addr, data: u8) {
            self.bytes[addr as usize] = data;
        }
    }

    impl_mem_32_little!{ LittleMemTest }

    #[test]
    fn memory() {
        let mut mem = LittleMemTest::new(0x100);

        mem.write_byte(0, 0x12);
        mem.write_byte(1, 0x34);
        mem.write_byte(2, 0x56);
        mem.write_byte(3, 0x78);

        assert_eq!(mem.read_word(0), 0x78563412);
    }
}
