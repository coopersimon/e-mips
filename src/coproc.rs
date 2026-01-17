use crate::cpu::ExceptionCode;

/// Trait for implementations of Coprocessors 1-3.
pub trait Coprocessor {
    fn move_from_reg(&mut self, reg: usize) -> u32;
    fn move_to_reg(&mut self, reg: usize, val: u32);

    fn move_from_control(&mut self, reg: usize) -> u32;
    fn move_to_control(&mut self, reg: usize, val: u32);

    fn load_from_mem(&mut self, reg: usize, val: u32);
    fn store_to_mem(&mut self, reg: usize) -> u32;

    fn operation(&mut self, op: u32);
}

pub struct EmptyCoproc {}

impl Coprocessor for EmptyCoproc {
    fn move_from_reg(&mut self, _: usize) -> u32 {
        0
    }
    fn move_to_reg(&mut self, _: usize, _: u32) {}

    fn move_from_control(&mut self, _: usize) -> u32 {
        0
    }
    fn move_to_control(&mut self, _: usize, _: u32) {}

    fn load_from_mem(&mut self, _: usize, _: u32) {}
    fn store_to_mem(&mut self, _: usize) -> u32 {
        0
    }

    fn operation(&mut self, _: u32) {}
}

/// Exception data for coprocessor 0.
pub struct Exception {
    pub code:               ExceptionCode,
    pub ret_addr:           u32,
    pub bad_virtual_addr:   u32,
    pub branch_delay:       bool,
}

/// Trait for implementations of Coprocessor 0.
///
/// This is a special coprocessor for handling of
/// exceptions, virtual memory, and other system ops.
pub trait Coprocessor0 {
    // Called by instructions
    fn move_from_reg(&mut self, reg: usize) -> u32;
    fn move_to_reg(&mut self, reg: usize, val: u32);

    fn operation(&mut self, op: u32);

    /// This will trigger an exception with the defined data.
    /// Should only be called from the CPU side.
    /// 
    /// Returns an exception vector to jump to.
    fn trigger_exception(&mut self, exception: &Exception) -> u32;
}

pub struct EmptyCoproc0 {}

impl Coprocessor0 for EmptyCoproc0 {
    fn move_from_reg(&mut self, _: usize) -> u32 {
        0
    }
    fn move_to_reg(&mut self, _: usize, _: u32) {}

    fn operation(&mut self, _: u32) {}

    fn trigger_exception(&mut self, _: &Exception) -> u32 {
        0
    }
}
