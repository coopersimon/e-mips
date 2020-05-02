pub trait Coprocessor: From<EmptyCoproc> {
    fn move_from_reg(&mut self, reg: usize) -> u32;
    fn move_to_reg(&mut self, reg: usize, val: u32);

    fn move_from_control(&mut self, reg: usize) -> u32;
    fn move_to_control(&mut self, reg: usize, val: u32);

    fn operation(&mut self, op: u32);
}

pub struct EmptyCoproc {}

impl Coprocessor for EmptyCoproc {
    fn move_from_reg(&mut self, reg: usize) -> u32 {
        0
    }
    fn move_to_reg(&mut self, reg: usize, val: u32) {}

    fn move_from_control(&mut self, reg: usize) -> u32 {
        0
    }
    fn move_to_control(&mut self, reg: usize, val: u32) {}

    fn operation(&mut self, op: u32) {}
}