/// Little-endian memory implementations.
mod little;

use num_traits::sign::Unsigned;

#[derive(Default, Clone, Copy)]
/// Address bus with variable width.
pub struct AddrBus<Width: Unsigned> {
    addr: Width
}

impl<Width: Unsigned> AddrBus<Width> {
    pub fn new(val: Width) -> Self {
        Self {
            addr: val
        }
    }

    pub fn inc(self) -> Self {
        AddrBus {
            addr: self.addr + Width::one()
        }
    }
}

/// Base memory trait.
///
/// When implementing a memory device, this trait must be implemented manually.
/// Then further memory traits can be implemented.
pub trait Memory {

    /// Address bus width. This describes the width of the address bus.
    /// In MIPS this is usually the same as the data width (i.e., 32 or 64-bits.)
    type Width: Unsigned + Copy;

    /// Read a single byte.
    fn read_byte(&mut self, addr: AddrBus<Self::Width>) -> u8;

    /// Write a single byte.
    fn write_byte(&mut self, addr: AddrBus<Self::Width>, data: u8);
}

/// Memory with a 16-bit data bus.
/// 
/// For default impls, see `impl_mem_16_little`.
pub trait Mem16: Memory {

    /// Read a 16-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_halfword(&mut self, addr: AddrBus<Self::Width>) -> u16;

    /// Write a 16-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_halfword(&mut self, addr: AddrBus<Self::Width>, data: u16);
}

/// Memory with a 32-bit data bus.
/// 
/// For default impls, see `impl_mem_32_little`.
pub trait Mem32: Mem16 {

    /// Read a 32-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_word(&mut self, addr: AddrBus<Self::Width>) -> u32;

    /// Write a 32-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_word(&mut self, addr: AddrBus<Self::Width>, data: u32);
}

/// Memory with a 64-bit data bus.
/// 
/// For default impls, see `impl_mem_64_little`.
pub trait Mem64: Mem32 {

    /// Read a 64-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_doubleword(&mut self, addr: AddrBus<Self::Width>) -> u64;

    /// Write a 64-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_doubleword(&mut self, addr: AddrBus<Self::Width>, data: u64);
}

/// This provides default implementations for the `Mem16` trait, however they are not very optimal
/// and you might get better performance from implementing them yourself.
#[macro_export]
macro_rules! impl_mem_16_little {
    {$struct:ident} => {
        impl Mem16 for $struct {
            fn read_halfword(&mut self, addr: AddrBus<Self::Width>) -> u16 {
                little::read_halfword(self, addr)
            }

            fn write_halfword(&mut self, addr: AddrBus<Self::Width>, data: u16) {
                little::write_halfword(self, addr, data);
            }
        }
    };
}

/// This provides default implementations for the `Mem32` and `Mem16` traits, however they are not very optimal
/// and you might get better performance from implementing them yourself.
#[macro_export]
macro_rules! impl_mem_32_little {
    {$struct:ident} => {
        impl_mem_16_little!{ $struct }

        impl Mem32 for $struct {
            fn read_word(&mut self, addr: AddrBus<Self::Width>) -> u32 {
                little::read_word(self, addr)
            }

            fn write_word(&mut self, addr: AddrBus<Self::Width>, data: u32) {
                little::write_word(self, addr, data);
            }
        }
    };
}

/// This provides default implementations for the `Mem64`, `Mem32` and `Mem16` traits, however they are not very optimal
/// and you might get better performance from implementing them yourself.
#[macro_export]
macro_rules! impl_mem_64_little {
    {$struct:ident} => {
        impl_mem_32_little!{ $struct }

        impl Mem64 for $struct {
            fn read_doubleword(&mut self, addr: AddrBus<Self::Width>) -> u64 {
                little::read_doubleword(self, addr)
            }

            fn write_doubleword(&mut self, addr: AddrBus<Self::Width>, data: u64) {
                little::write_doubleword(self, addr, data);
            }
        }
    };
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
        type Width = u32;

        fn read_byte(&mut self, addr: AddrBus<Self::Width>) -> u8 {
            self.bytes[addr.addr as usize]
        }

        fn write_byte(&mut self, addr: AddrBus<Self::Width>, data: u8) {
            self.bytes[addr.addr as usize] = data;
        }
    }

    impl_mem_32_little!{ LittleMemTest }

    #[test]
    fn memory() {
        let mut mem = LittleMemTest::new(0x100);

        mem.write_byte(AddrBus::new(0), 0x12);
        mem.write_byte(AddrBus::new(1), 0x34);
        mem.write_byte(AddrBus::new(2), 0x56);
        mem.write_byte(AddrBus::new(3), 0x78);

        assert_eq!(mem.read_word(AddrBus::new(0)), 0x78563412);
    }
}
