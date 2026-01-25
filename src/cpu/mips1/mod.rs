mod instructions;
#[cfg(test)]
mod mips1_test;

use super::*;

use crate::common::*;
use crate::coproc::*;
use crate::mem::{
    Mem32
};

pub use instructions::*;

/// Mips I processor.
pub struct MIPSI<
    Mem: Mem32 + Coprocessor0,
    C1: Coprocessor = EmptyCoproc,
    C2: Coprocessor = EmptyCoproc,
    C3: Coprocessor = EmptyCoproc
> {
    gp_reg:     [u32; 32],
    hi:         u32,
    lo:         u32,

    /// Program counter of the next instruction.
    pc:             u32,
    /// Program counter of the instruction after.
    pc_next:        u32,
    /// This is used when exceptions occur.
    current_instr_addr: u32,
    /// The current instruction is in the branch delay slot.
    branch_delay:       bool,
    /// An interrupt occurred in the branch delay slot.
    branch_interrupt:   bool,

    mem:        Box<Mem>,

    coproc1:    Option<C1>,
    coproc2:    Option<C2>,
    coproc3:    Option<C3>
}

impl<
    Mem: Mem32 + Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSI<Mem, C1, C2, C3> {
    /// Make a new MIPS I processor.
    fn new(mem: Box<Mem>, coproc1: Option<C1>, coproc2: Option<C2>, coproc3: Option<C3>) -> Self {
        Self {
            gp_reg:     [0; 32],
            hi:         0,
            lo:         0,

            pc:         0,
            pc_next:    4,
            current_instr_addr: 0,
            branch_delay:       false,
            branch_interrupt:   false,

            mem:        mem,

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
    Mem: Mem32 + Coprocessor0,
    C1: Coprocessor = EmptyCoproc,
    C2: Coprocessor = EmptyCoproc,
    C3: Coprocessor = EmptyCoproc
> {
    mem:        Box<Mem>,

    coproc1:    Option<C1>,
    coproc2:    Option<C2>,
    coproc3:    Option<C3>,
}

impl<
    Mem: Mem32 + Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSIBuilder<Mem, C1, C2, C3> {
    fn new(mem: Box<Mem>) -> MIPSIBuilder<Mem, EmptyCoproc, EmptyCoproc, EmptyCoproc> {
        MIPSIBuilder {
            mem:        mem,
            coproc1:    None,
            coproc2:    None,
            coproc3:    None,
        }
    }

    /// Add a coprocessor to slot 1.
    pub fn add_coproc1<NewC1: Coprocessor>(self, coproc1: NewC1) -> MIPSIBuilder<Mem, NewC1, C2, C3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc1:    Some(coproc1),
            coproc2:    self.coproc2,
            coproc3:    self.coproc3,
        }
    }

    /// Add a coprocessor to slot 2.
    pub fn add_coproc2<NewC2: Coprocessor>(self, coproc2: NewC2) -> MIPSIBuilder<Mem, C1, NewC2, C3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc1:    self.coproc1,
            coproc2:    Some(coproc2),
            coproc3:    self.coproc3,
        }
    }

    /// Add a coprocessor to slot 3.
    pub fn add_coproc3<NewC3: Coprocessor>(self, coproc3: NewC3) -> MIPSIBuilder<Mem, C1, C2, NewC3> {
        MIPSIBuilder {
            mem:        self.mem,
            coproc1:    self.coproc1,
            coproc2:    self.coproc2,
            coproc3:    Some(coproc3),
        }
    }

    /// Make the MIPS I processor.
    pub fn build(self) -> MIPSI<Mem, C1, C2, C3> {
        MIPSI::new(self.mem, self.coproc1, self.coproc2, self.coproc3)
    }
}

impl<
    Mem: Mem32<Addr = u32> + Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSICore for MIPSI<Mem, C1, C2, C3> {
    type Mem = Mem;
    type Coproc0 = Mem;
    type Coproc1 = C1;
    type Coproc2 = C2;
    type Coproc3 = C3;

    fn read_gp(&self, reg: u8) -> u32 {
        self.gp_reg[reg as usize]
    }
    fn write_gp(&mut self, reg: u8, val: u32) {
        if reg != 0 {
            self.gp_reg[reg as usize] = val;
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

    fn link_register(&mut self, reg: u8) {
        self.write_gp(reg, self.pc_next);
    }

    fn branch(&mut self, offset: u32) {
        self.branch_delay = true;
        self.pc_next = self.pc.wrapping_add(offset);
    }

    fn jump_global(&mut self, addr: u32) {
        self.branch_delay = true;
        self.pc_next = addr;
    }

    fn jump_segment(&mut self, segment_addr: u32) {
        self.branch_delay = true;
        let hi = self.pc_next & 0xF000_0000;
        self.pc_next = hi | segment_addr;
    }

    fn trigger_exception(&mut self, exception: ExceptionCode) {
        let exception = Exception {
            code: exception,
            ret_addr: if self.branch_delay {self.current_instr_addr.wrapping_sub(4)} else {self.current_instr_addr},
            bad_virtual_addr: 0, // TODO!
            branch_delay: self.branch_delay,
        };
        self.pc = self.coproc_0().trigger_exception(&exception);
        self.pc_next = self.pc.wrapping_add(4);
    }

    fn mut_mem<'a>(&'a mut self) -> &'a mut Self::Mem {
        &mut self.mem
    }

    fn read_pc(&self) -> u32 {
        self.pc
    }

    fn coproc_0<'a>(&'a mut self) -> &'a mut Self::Coproc0 {
        &mut self.mem
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
    Mem: Mem32<Addr = u32> + Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSIInstructions<Mem> for MIPSI<Mem, C1, C2, C3> {}
