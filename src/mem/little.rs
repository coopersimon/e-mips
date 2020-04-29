/// Little endian memory traits.

use super::{
    AddrBus,
    Memory
};

/// Read a 16-bit value, using little-endianness.
/// 
/// Reads from this can be expected to be aligned (the bottom addr bit should be 0).
/// Unaligned reads are undefined, and might panic.
pub fn read_halfword<M: Memory>(mem: &mut M, addr: AddrBus<M::Width>) -> u16 {
    let lo = mem.read_byte(addr);
    let hi = mem.read_byte(addr.inc());
    make16!(hi, lo)
}

/// Write a 16-bit value, using little-endianness.
/// 
/// Writes to this can be expected to be aligned (the bottom addr bit should be 0).
/// Unaligned writes are undefined, and might panic.
pub fn write_halfword<M: Memory>(mem: &mut M, addr: AddrBus<M::Width>, data: u16) {
    let lo = lo16!(data);
    let hi = hi16!(data);
    mem.write_byte(addr, lo);
    mem.write_byte(addr.inc(), hi);
}

/// Read a 32-bit value, using little-endianness.
/// 
/// Reads from this can be expected to be aligned (the bottom 2 addr bits should be 0).
/// Unaligned reads are undefined, and might panic.
pub fn read_word<M: Memory>(mem: &mut M, addr: AddrBus<M::Width>) -> u32 {
    let addr0 = addr;
    let addr1 = addr0.inc();
    let addr2 = addr1.inc();
    let addr3 = addr2.inc();
    let b0 = mem.read_byte(addr0);
    let b1 = mem.read_byte(addr1);
    let b2 = mem.read_byte(addr2);
    let b3 = mem.read_byte(addr3);
    make32!(b3, b2, b1, b0)
}

/// Write a 32-bit value, using little-endianness.
/// 
/// Writes to this can be expected to be aligned (the bottom 2 addr bits should be 0).
/// Unaligned writes are undefined, and might panic.
pub fn write_word<M: Memory>(mem: &mut M, addr: AddrBus<M::Width>, data: u32) {
    let bytes = bytes32!(data);
    let addr0 = addr;
    let addr1 = addr0.inc();
    let addr2 = addr1.inc();
    let addr3 = addr2.inc();
    mem.write_byte(addr0, bytes.0);
    mem.write_byte(addr1, bytes.1);
    mem.write_byte(addr2, bytes.2);
    mem.write_byte(addr3, bytes.3);
}

/// Read a 64-bit value, using little-endianness.
/// 
/// Reads from this can be expected to be aligned (the bottom 3 addr bits should be 0).
/// Unaligned reads are undefined, and might panic.
pub fn read_doubleword<M: Memory>(mem: &mut M, addr: AddrBus<M::Width>) -> u64 {
    let addr0 = addr;
    let addr1 = addr0.inc();
    let addr2 = addr1.inc();
    let addr3 = addr2.inc();
    let addr4 = addr3.inc();
    let addr5 = addr4.inc();
    let addr6 = addr5.inc();
    let addr7 = addr6.inc();
    let b0 = mem.read_byte(addr0);
    let b1 = mem.read_byte(addr1);
    let b2 = mem.read_byte(addr2);
    let b3 = mem.read_byte(addr3);
    let b4 = mem.read_byte(addr4);
    let b5 = mem.read_byte(addr5);
    let b6 = mem.read_byte(addr6);
    let b7 = mem.read_byte(addr7);
    make64!(b7, b6, b5, b4, b3, b2, b1, b0)
}

/// Write a 64-bit value, using little-endianness.
/// 
/// Writes to this can be expected to be aligned (the bottom 3 addr bits should be 0).
/// Unaligned writes are undefined, and might panic.
pub fn write_doubleword<M: Memory>(mem: &mut M, addr: AddrBus<M::Width>, data: u64) {
    let bytes = bytes64!(data);
    let addr0 = addr;
    let addr1 = addr0.inc();
    let addr2 = addr1.inc();
    let addr3 = addr2.inc();
    let addr4 = addr3.inc();
    let addr5 = addr4.inc();
    let addr6 = addr5.inc();
    let addr7 = addr6.inc();
    mem.write_byte(addr0, bytes.0);
    mem.write_byte(addr1, bytes.1);
    mem.write_byte(addr2, bytes.2);
    mem.write_byte(addr3, bytes.3);
    mem.write_byte(addr4, bytes.4);
    mem.write_byte(addr5, bytes.5);
    mem.write_byte(addr6, bytes.6);
    mem.write_byte(addr7, bytes.7);
}
