mod instructions;
#[cfg(test)]
mod mips1_test;

use super::*;

use crate::common::*;
use crate::coproc::*;
use crate::mem::{
    Memory,
    Mem16,
    Mem32
};

pub use instructions::*;

/// Mips I processor.
pub struct MIPSI<
    Mem: Mem32,
    C0: Coprocessor0 = EmptyCoproc0,
    C1: Coprocessor = EmptyCoproc,
    C2: Coprocessor = EmptyCoproc,
    C3: Coprocessor = EmptyCoproc
> {
    gp_reg:     [u32; 32],
    hi:         u32,
    lo:         u32,

    pc:         u32,
    pc_next:    u32,

    mem:        Box<Mem>,

    coproc0:    Option<C0>,
    coproc1:    Option<C1>,
    coproc2:    Option<C2>,
    coproc3:    Option<C3>
}

impl<
    Mem: Mem32,
    C0: Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSI<Mem, C0, C1, C2, C3> {
    /// Make a new MIPS I processor.
    fn new(mem: Box<Mem>, coproc0: Option<C0>, coproc1: Option<C1>, coproc2: Option<C2>, coproc3: Option<C3>) -> Self {
        Self {
            gp_reg:     [0; 32],
            hi:         0,
            lo:         0,

            pc:         0,
            pc_next:    4,

            mem:        mem,

            coproc0:    coproc0,
            coproc1:    coproc1,
            coproc2:    coproc2,
            coproc3:    coproc3,
        }
    }

    /// Make a new MIPS I processor.
    /// 
    /// Use the builder provided to add any coprocessors desired,
    /// then call `build` to finish.
    pub fn with_memory(mem: Box<Mem>) -> MIPSIBuilder<Mem> {
        MIPSIBuilder::<Mem>::new(mem)
    }
}

//
pub struct MIPSIBuilder<
    Mem: Mem32,
    C0: Coprocessor0 = EmptyCoproc0,
    C1: Coprocessor = EmptyCoproc,
    C2: Coprocessor = EmptyCoproc,
    C3: Coprocessor = EmptyCoproc
> {
    mem:        Box<Mem>,

    coproc0:    Option<C0>,
    coproc1:    Option<C1>,
    coproc2:    Option<C2>,
    coproc3:    Option<C3>,
}

impl<
    Mem: Mem32,
    C0: Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSIBuilder<Mem, C0, C1, C2, C3> {
    fn new(mem: Box<Mem>) -> MIPSIBuilder<Mem, EmptyCoproc0, EmptyCoproc, EmptyCoproc, EmptyCoproc> {
        MIPSIBuilder {
            mem:        mem,
            coproc0:    None,
            coproc1:    None,
            coproc2:    None,
            coproc3:    None,
        }
    }

    /// Add a coprocessor to slot 0.
    pub fn add_coproc0<NewC0: Coprocessor0>(self, coproc0: NewC0) -> MIPSIBuilder<Mem, NewC0, C1, C2, C3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc0:    Some(coproc0),
            coproc1:    self.coproc1,
            coproc2:    self.coproc2,
            coproc3:    self.coproc3,
        }
    }

    /// Add a coprocessor to slot 1.
    pub fn add_coproc1<NewC1: Coprocessor>(self, coproc1: NewC1) -> MIPSIBuilder<Mem, C0, NewC1, C2, C3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc0:    self.coproc0,
            coproc1:    Some(coproc1),
            coproc2:    self.coproc2,
            coproc3:    self.coproc3,
        }
    }

    /// Add a coprocessor to slot 2.
    pub fn add_coproc2<NewC2: Coprocessor>(self, coproc2: NewC2) -> MIPSIBuilder<Mem, C0, C1, NewC2, C3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc0:    self.coproc0,
            coproc1:    self.coproc1,
            coproc2:    Some(coproc2),
            coproc3:    self.coproc3,
        }
    }

    /// Add a coprocessor to slot 3.
    pub fn add_coproc3<NewC3: Coprocessor>(self, coproc3: NewC3) -> MIPSIBuilder<Mem, C0, C1, C2, NewC3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc0:    self.coproc0,
            coproc1:    self.coproc1,
            coproc2:    self.coproc2,
            coproc3:    Some(coproc3),
        }
    }

    /// Make the MIPS I processor.
    pub fn build(self) -> MIPSI<Mem, C0, C1, C2, C3> {
        MIPSI::new(self.mem, self.coproc0, self.coproc1, self.coproc2, self.coproc3)
    }
}

impl<
    Mem: Mem32<Addr = u32>,
    C0: Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSICore for MIPSI<Mem, C0, C1, C2, C3> {
    type Mem = Mem;
    type Coproc0 = C0;
    type Coproc1 = C1;
    type Coproc2 = C2;
    type Coproc3 = C3;

    fn read_gp(&self, reg: usize) -> u32 {
        self.gp_reg[reg]
    }
    fn write_gp(&mut self, reg: usize, val: u32) {
        if reg != 0 {
            self.gp_reg[reg] = val;
        }
    }

    fn read_hi(&self) -> u32 {
        self.hi
    }
    fn write_hi(&mut self, val: u32) {
        self.hi = val;
    }

    fn read_lo(&self) -> u32 {
        self.lo
    }
    fn write_lo(&mut self, val: u32) {
        self.lo = val;
    }

    fn link_register(&mut self, reg: usize) {
        self.write_gp(reg, self.pc_next);
    }

    fn branch(&mut self, offset: u32) {
        self.pc_next = self.pc.wrapping_add(offset);
    }

    fn jump(&mut self, segment_addr: u32) {
        let hi = self.pc_next & 0xF000_0000;
        self.pc_next = hi | segment_addr;
    }

    fn trigger_exception(&mut self, exception: ExceptionCode) {

    }

    fn mem<'a>(&'a mut self) -> &'a mut Self::Mem {
        &mut self.mem
    }

    fn coproc_0<'a>(&'a mut self) -> Option<&'a mut Self::Coproc0> {
        (&mut self.coproc0).as_mut()
    }

    fn coproc_1<'a>(&'a mut self) -> Option<&'a mut Self::Coproc1> {
        (&mut self.coproc1).as_mut()
    }

    fn coproc_2<'a>(&'a mut self) -> Option<&'a mut Self::Coproc2> {
        (&mut self.coproc2).as_mut()
    }

    fn coproc_3<'a>(&'a mut self) -> Option<&'a mut Self::Coproc3> {
        (&mut self.coproc3).as_mut()
    }
}

impl<
    Mem: Mem32<Addr = u32>,
    C0: Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSIInstructions<Mem> for MIPSI<Mem, C0, C1, C2, C3> {}
