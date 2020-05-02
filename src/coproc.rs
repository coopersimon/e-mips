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

pub trait Coprocessor0 {
    fn move_from_reg(&mut self, reg: usize) -> u32;
    fn move_to_reg(&mut self, reg: usize, val: u32);

    fn operation(&mut self, op: u32);
}

pub struct EmptyCoproc0 {}

impl Coprocessor0 for EmptyCoproc0 {
    fn move_from_reg(&mut self, _: usize) -> u32 {
        0
    }
    fn move_to_reg(&mut self, _: usize, _: u32) {}

    fn operation(&mut self, _: u32) {}
}
