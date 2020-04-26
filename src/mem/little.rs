/// Little endian memory traits.

use num_traits::sign::Unsigned;
use super::{
    AddrBus,
    Memory
};

/// Little-endian memory with a 16-bit data bus.
/// 
/// This trait provides default implementations, however they are not very optimal
/// and you might get better performance from implementing them yourself.
pub trait LittleMem16<Width: Unsigned + Copy>: Memory<Width> {
    /// Read a 16-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_halfword(&mut self, addr: AddrBus<Width>) -> u16 {
        let lo = self.read_byte(addr);
        let hi = self.read_byte(addr.inc());
        make16!(hi, lo)
    }

    /// Write a 16-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom addr bit should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_halfword(&mut self, addr: AddrBus<Width>, data: u16) {
        let lo = lo16!(data);
        let hi = hi16!(data);
        self.write_byte(addr, lo);
        self.write_byte(addr.inc(), hi);
    }
}

/// Little-endian memory with a 32-bit data bus.
/// 
/// This trait provides default implementations, however they are not very optimal
/// and you might get better performance from implementing them yourself.
pub trait LittleMem32<Width: Unsigned + Copy>: LittleMem16<Width> {
    /// Read a 32-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_word(&mut self, addr: AddrBus<Width>) -> u32 {
        let addr0 = addr;
        let addr1 = addr0.inc();
        let addr2 = addr1.inc();
        let addr3 = addr2.inc();
        let b0 = self.read_byte(addr0);
        let b1 = self.read_byte(addr1);
        let b2 = self.read_byte(addr2);
        let b3 = self.read_byte(addr3);
        make32!(b3, b2, b1, b0)
    }

    /// Write a 32-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 2 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_word(&mut self, addr: AddrBus<Width>, data: u32) {
        let bytes = bytes32!(data);
        let addr0 = addr;
        let addr1 = addr0.inc();
        let addr2 = addr1.inc();
        let addr3 = addr2.inc();
        self.write_byte(addr0, bytes.0);
        self.write_byte(addr1, bytes.1);
        self.write_byte(addr2, bytes.2);
        self.write_byte(addr3, bytes.3);
    }
}

/// Little-endian memory with a 32-bit data bus.
/// 
/// This trait provides default implementations, however they are not very optimal
/// and you might get better performance from implementing them yourself.
pub trait LittleMem64<Width: Unsigned + Copy>: LittleMem32<Width> {
    /// Read a 64-bit value.
    /// 
    /// Reads from this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned reads are undefined, and might panic.
    fn read_doubleword(&mut self, addr: AddrBus<Width>) -> u64 {
        let addr0 = addr;
        let addr1 = addr0.inc();
        let addr2 = addr1.inc();
        let addr3 = addr2.inc();
        let addr4 = addr3.inc();
        let addr5 = addr4.inc();
        let addr6 = addr5.inc();
        let addr7 = addr6.inc();
        let b0 = self.read_byte(addr0);
        let b1 = self.read_byte(addr1);
        let b2 = self.read_byte(addr2);
        let b3 = self.read_byte(addr3);
        let b4 = self.read_byte(addr4);
        let b5 = self.read_byte(addr5);
        let b6 = self.read_byte(addr6);
        let b7 = self.read_byte(addr7);
        make64!(b7, b6, b5, b4, b3, b2, b1, b0)
    }

    /// Write a 64-bit value.
    /// 
    /// Writes to this can be expected to be aligned (the bottom 3 addr bits should be 0).
    /// Unaligned writes are undefined, and might panic.
    fn write_doubleword(&mut self, addr: AddrBus<Width>, data: u64) {
        let bytes = bytes64!(data);
        let addr0 = addr;
        let addr1 = addr0.inc();
        let addr2 = addr1.inc();
        let addr3 = addr2.inc();
        let addr4 = addr3.inc();
        let addr5 = addr4.inc();
        let addr6 = addr5.inc();
        let addr7 = addr6.inc();
        self.write_byte(addr0, bytes.0);
        self.write_byte(addr1, bytes.1);
        self.write_byte(addr2, bytes.2);
        self.write_byte(addr3, bytes.3);
        self.write_byte(addr4, bytes.4);
        self.write_byte(addr5, bytes.5);
        self.write_byte(addr6, bytes.6);
        self.write_byte(addr7, bytes.7);
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

    impl Memory<u32> for LittleMemTest {
        fn read_byte(&mut self, addr: AddrBus<u32>) -> u8 {
            self.bytes[addr.addr as usize]
        }

        fn write_byte(&mut self, addr: AddrBus<u32>, data: u8) {
            self.bytes[addr.addr as usize] = data;
        }
    }

    impl LittleMem16<u32> for LittleMemTest {}
    impl LittleMem32<u32> for LittleMemTest {}

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