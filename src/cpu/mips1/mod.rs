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
    C0: Coprocessor,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> {
    gp_reg:     [u32; 32],
    hi:         u32,
    lo:         u32,

    pc:         u32,
    pc_next:    u32,

    mem:        Box<Mem>,

    coproc0:    C0,
    coproc1:    C1,
    coproc2:    C2,
    coproc3:    C3
}

impl<
    Mem: Mem32,
    C0: Coprocessor,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSI<Mem, C0, C1, C2, C3> {
    /// Make a new MIPS I processor.
    fn new(mem: Box<Mem>, coproc0: C0, coproc1: C1, coproc2: C2, coproc3: C3) -> Self {
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

    pub fn with_memory(mem: Box<Mem>) -> MIPSIBuilder<Mem, C0, C1, C2, C3> {
        MIPSIBuilder::new(mem)
    }
}

//
pub struct MIPSIBuilder<
    Mem: Mem32,
    C0: Coprocessor,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> {
    mem:        Box<Mem>,

    coproc0:    Option<C0>,
    coproc1:    Option<C1>,
    coproc2:    Option<C2>,
    coproc3:    Option<C3>,
}

impl<
    Mem: Mem32,
    C0In: Coprocessor,
    C1In: Coprocessor,
    C2In: Coprocessor,
    C3In: Coprocessor
> MIPSIBuilder<Mem, C0In, C1In, C2In, C3In> {
    fn new(mem: Box<Mem>) -> Self {
        Self {
            mem:        mem,
            coproc0:    None,
            coproc1:    None,
            coproc2:    None,
            coproc3:    None,
        }
    }

    pub fn add_coproc0(mut self, coproc0: C0In) -> Self {
        self.coproc0 = Some(coproc0);
        self
    }

    pub fn add_coproc1(mut self, coproc1: C1In) -> Self {
        self.coproc1 = Some(coproc1);
        self
    }

    pub fn add_coproc2(mut self, coproc2: C2In) -> Self {
        self.coproc2 = Some(coproc2);
        self
    }

    pub fn add_coproc3(mut self, coproc3: C3In) -> Self {
        self.coproc3 = Some(coproc3);
        self
    }

    pub fn build<
        C0Out: Coprocessor + From<C0In> + From<EmptyCoproc>,
        C1Out: Coprocessor + From<C1In> + From<EmptyCoproc>,
        C2Out: Coprocessor + From<C2In> + From<EmptyCoproc>,
        C3Out: Coprocessor + From<C3In> + From<EmptyCoproc>
    >(self) -> MIPSI<Mem, C0Out, C1Out, C2Out, C3Out> {
        let coproc0 = if let Some(c0) = self.coproc0 {c0.into()} else {EmptyCoproc{}.into()};
        let coproc1 = if let Some(c1) = self.coproc1 {c1.into()} else {EmptyCoproc{}.into()};
        let coproc2 = if let Some(c2) = self.coproc2 {c2.into()} else {EmptyCoproc{}.into()};
        let coproc3 = if let Some(c3) = self.coproc3 {c3.into()} else {EmptyCoproc{}.into()};

        MIPSI::new(self.mem, coproc0, coproc1, coproc2, coproc3)
    }
}

impl<
    Mem: Mem32<Addr = u32>,
    C0: Coprocessor,
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

    fn coproc_0<'a>(&'a mut self) -> &'a mut Self::Coproc0 {
        &mut self.coproc0
    }

    fn coproc_1<'a>(&'a mut self) -> &'a mut Self::Coproc1 {
        &mut self.coproc1
    }

    fn coproc_2<'a>(&'a mut self) -> &'a mut Self::Coproc2 {
        &mut self.coproc2
    }

    fn coproc_3<'a>(&'a mut self) -> &'a mut Self::Coproc3 {
        &mut self.coproc3
    }
}

impl<
    Mem: Mem32<Addr = u32>,
    C0: Coprocessor,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSIInstructions<Mem> for MIPSI<Mem, C0, C1, C2, C3> {}
