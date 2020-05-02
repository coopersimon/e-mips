/// The MIPS I instruction set.
pub mod mips1;
#[cfg(test)]
mod mips1_test;

use crate::mem::Mem32;
use crate::coproc::Coprocessor;

/// Exception codes.
pub enum ExceptionCode {
    Interrupt           = 0,
    TLBMod              = 1,
    TLBLoad             = 2,
    TLBStore            = 3,
    AddrErrorLoad       = 4,
    AddrErrorStore      = 5,
    InstructionBusError = 6,
    DataBusError        = 7,
    Syscall             = 8,
    Breakpoint          = 9,
    ReservedInstruction = 10,
    CoProcUnusable      = 11,
    ArithmeticOverflow  = 12
}

/// The core set of traits for a MIPS processor.
/// 
/// This set of traits deals with the public interface.
pub trait MIPSCore {
    /// Fetch, decode, and execute an instruction.
    fn step(&mut self);
}

/// The core set of traits for the MIPS I instruction set.
/// 
/// This set of traits wraps:
/// - The core register access
/// - Exception handling
pub trait MIPSICore {
    /// The memory bus.
    type Mem: Mem32;
    /// The type for Coprocessor 0.
    type Coproc0: Coprocessor;
    /// The type for Coprocessor 1.
    type Coproc1: Coprocessor;
    /// The type for Coprocessor 2.
    type Coproc2: Coprocessor;
    /// The type for Coprocessor 3.
    type Coproc3: Coprocessor;

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

    /// Link the specified register with the return address.
    fn link_register(&mut self, reg: usize);

    /// Modify the next PC (in the case of a branch).
    fn branch(&mut self, offset: u32);

    /// Modify the next PC (in the case of a jump).
    fn jump(&mut self, segment_addr: u32);

    /// Trigger an exception.
    fn trigger_exception(&mut self, exception: ExceptionCode);

    /// Borrow the memory bus.
    fn mem<'a>(&'a mut self) -> &'a mut Self::Mem;

    /// Borrow coprocessor 0.
    fn coproc_0<'a>(&'a mut self) -> &'a mut Self::Coproc0;
    /// Borrow coprocessor 1.
    fn coproc_1<'a>(&'a mut self) -> &'a mut Self::Coproc1;
    /// Borrow coprocessor 2.
    fn coproc_2<'a>(&'a mut self) -> &'a mut Self::Coproc2;
    /// Borrow coprocessor 3.
    fn coproc_3<'a>(&'a mut self) -> &'a mut Self::Coproc3;
}
