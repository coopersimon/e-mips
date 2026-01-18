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

/// Memory with a 32-bit data bus.
pub trait Mem32 {

    /// This type describes the width of the address bus.
    /// In MIPS this is usually the same as the data width (i.e., 32 or 64-bits.)
    type Addr: Unsigned + Copy;

    /// Check the endianness of this memory.
    /// 
    /// If this is `true`, the memory is little-endian.
    /// If this is `false`, the memory is big-endian.
    const LITTLE_ENDIAN: bool;

    /// Inform the memory bus that cycles have passed.
    /// 
    /// Returns any interrupt bits that were set.
    fn clock(&mut self, cycles: usize) -> u8;

    /// Read a single byte.
    fn read_byte(&mut self, addr: Self::Addr) -> u8;

    /// Write a single byte.
    fn write_byte(&mut self, addr: Self::Addr, data: u8);

    /// Read a 16-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_halfword(&mut self, addr: Self::Addr) -> u16 {
        use num_traits::identities::One;

        let lo = self.read_byte(addr);
        let hi = self.read_byte(addr + Self::Addr::one());
        if Self::LITTLE_ENDIAN {
            u16::from_le_bytes([lo, hi])
        } else {
            u16::from_be_bytes([lo, hi])
        }
    }

    /// Write a 16-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_halfword(&mut self, addr: Self::Addr, data: u16) {
        use num_traits::identities::One;

        let [lo, hi] = if Self::LITTLE_ENDIAN {
            data.to_le_bytes()
        } else {
            data.to_be_bytes()
        };
        self.write_byte(addr, lo);
        self.write_byte(addr + Self::Addr::one(), hi);
    }

    /// Read a 32-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_word(&mut self, addr: Self::Addr) -> u32 {
        use num_traits::identities::One;

        let addr0 = addr;
        let addr1 = addr0 + Self::Addr::one();
        let addr2 = addr1 + Self::Addr::one();
        let addr3 = addr2 + Self::Addr::one();
        let b0 = self.read_byte(addr0);
        let b1 = self.read_byte(addr1);
        let b2 = self.read_byte(addr2);
        let b3 = self.read_byte(addr3);
        if Self::LITTLE_ENDIAN {
            u32::from_le_bytes([b0, b1, b2, b3])
        } else {
            u32::from_be_bytes([b0, b1, b2, b3])
        }
    }

    /// Write a 32-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_word(&mut self, addr: Self::Addr, data: u32) {
        use num_traits::identities::One;

        let bytes = if Self::LITTLE_ENDIAN {
            data.to_le_bytes()
        } else {
            data.to_be_bytes()
        };
        let addr0 = addr;
        let addr1 = addr0 + Self::Addr::one();
        let addr2 = addr1 + Self::Addr::one();
        let addr3 = addr2 + Self::Addr::one();
        self.write_byte(addr0, bytes[0]);
        self.write_byte(addr1, bytes[1]);
        self.write_byte(addr2, bytes[2]);
        self.write_byte(addr3, bytes[3]);
    }
}

/// Memory with a 64-bit data bus.
pub trait Mem64: Mem32 {

    /// Read a 64-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_doubleword(&mut self, addr: Self::Addr) -> u64 {
        use num_traits::identities::One;

        let addr0 = addr;
        let addr1 = addr0 + Self::Addr::one();
        let addr2 = addr1 + Self::Addr::one();
        let addr3 = addr2 + Self::Addr::one();
        let addr4 = addr3 + Self::Addr::one();
        let addr5 = addr4 + Self::Addr::one();
        let addr6 = addr5 + Self::Addr::one();
        let addr7 = addr6 + Self::Addr::one();
        let b0 = self.read_byte(addr0);
        let b1 = self.read_byte(addr1);
        let b2 = self.read_byte(addr2);
        let b3 = self.read_byte(addr3);
        let b4 = self.read_byte(addr4);
        let b5 = self.read_byte(addr5);
        let b6 = self.read_byte(addr6);
        let b7 = self.read_byte(addr7);
        if Self::LITTLE_ENDIAN {
            u64::from_le_bytes([b0, b1, b2, b3, b4, b5, b6, b7])
        } else {
            u64::from_be_bytes([b0, b1, b2, b3, b4, b5, b6, b7])
        }
    }

    /// Write a 64-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_doubleword(&mut self, addr: Self::Addr, data: u64) {
        use num_traits::identities::One;

        let bytes = if Self::LITTLE_ENDIAN {
            data.to_le_bytes()
        } else {
            data.to_be_bytes()
        };
        let addr0 = addr;
        let addr1 = addr0 + Self::Addr::one();
        let addr2 = addr1 + Self::Addr::one();
        let addr3 = addr2 + Self::Addr::one();
        let addr4 = addr3 + Self::Addr::one();
        let addr5 = addr4 + Self::Addr::one();
        let addr6 = addr5 + Self::Addr::one();
        let addr7 = addr6 + Self::Addr::one();
        self.write_byte(addr0, bytes[0]);
        self.write_byte(addr1, bytes[1]);
        self.write_byte(addr2, bytes[2]);
        self.write_byte(addr3, bytes[3]);
        self.write_byte(addr4, bytes[4]);
        self.write_byte(addr5, bytes[5]);
        self.write_byte(addr6, bytes[6]);
        self.write_byte(addr7, bytes[7]);
    }
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

    impl Mem32 for LittleMemTest {
        type Addr = u32;
        const LITTLE_ENDIAN: bool = true;

        fn clock(&mut self, _cycles: usize) -> u8 {
            0
        }

        fn read_byte(&mut self, addr: Self::Addr) -> u8 {
            self.bytes[addr as usize]
        }

        fn write_byte(&mut self, addr: Self::Addr, data: u8) {
            self.bytes[addr as usize] = data;
        }
    }

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
