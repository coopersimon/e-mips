/// The MIPS I instruction set.
mod mips1;

/// Exceptions that can trigger internally.
pub enum Exception {
    ArithmeticOverflow = 12
}

/// The core set of traits for the MIPS I instruction set.
/// 
/// This set of traits wraps:
/// - The core register access
/// - Exception handling
pub trait MIPSICore {
    /// Read a general-purpose register.
    /// 
    /// If a read is attempted on a register that is not value 0-31,
    /// the results are undefined.
    fn read_gp(&self, reg: usize) -> u32;

    /// Write a general-purpose register.
    /// 
    /// If a write is attempted on a register that is not value 0-31,
    /// the results are undefined.
    fn write_gp(&mut self, reg: usize, val: u32);

    /// Read the HI register.
    fn read_hi(&self) -> u32;

    /// Write the HI register.
    fn write_hi(&mut self, val: u32);

    /// Read the LO register.
    fn read_lo(&self) -> u32;

    /// Write the LO register.
    fn write_lo(&mut self, val: u32);

    /// Trigger an exception.
    fn trigger_exception(&mut self, exception: Exception);
}
