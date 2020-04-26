/// Little-endian memory traits.
mod little;

use num_traits::sign::Unsigned;

pub use little::*;

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
/// 
/// The width generic parameter describes the width of the address bus.
/// In MIPS this is usually the same as the data width (i.e., 32 or 64-bits.)
pub trait Memory<Width: Unsigned + Copy> {
    /// Read a single byte.
    fn read_byte(&mut self, addr: AddrBus<Width>) -> u8;

    /// Write a single byte.
    fn write_byte(&mut self, addr: AddrBus<Width>, data: u8);
}
