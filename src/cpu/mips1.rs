use super::*;

use crate::common::*;
use crate::mem::{
    Memory,
    Mem16,
    Mem32
};

/// Mips I processor.
pub struct MIPSI<Mem: Mem32> {
    gp_reg:     [u32; 32],
    hi:         u32,
    lo:         u32,

    pc:         u32,
    pc_next:    u32,

    mem:        Mem
}

impl<Mem: Mem32> MIPSI<Mem> {
    pub fn new(mem: Mem) -> Self {
        Self {
            gp_reg:     [0; 32],
            hi:         0,
            lo:         0,

            pc:         0,
            pc_next:    4,

            mem:        mem,
        }
    }
}

impl<Mem: Mem32<Addr = u32>> MIPSICore for MIPSI<Mem> {
    type Mem = Mem;

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

    fn trigger_exception(&mut self, exception: ExceptionCode) {

    }

    fn mem<'a>(&'a mut self) -> &'a mut Self::Mem {
        &mut self.mem
    }
}

impl<Mem: Mem32<Addr = u32>> MIPSIInstructions<Mem> for MIPSI<Mem> {}

/// The set of instructions defined in MIPS I.
/// 
/// The arguments must have been decoded prior to calling these.
/// If a register number argument has a value greater than 31, the result is undefined.
pub trait MIPSIInstructions<Mem>: MIPSICore<Mem = Mem>
    where Mem: Mem32, <Mem as Memory>::Addr: From<u32> {
    // Arithmetic

    /// Add signed
    fn add(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_add(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(ExceptionCode::ArithmeticOverflow);
        }
    }

    /// Add immediate signed
    fn addi(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16(imm);
        if let Some(result) = source.checked_add(imm_32) {
            self.write_gp(tgt_reg, result);
        } else {
            self.trigger_exception(ExceptionCode::ArithmeticOverflow);
        }
    }

    /// Add unsigned
    fn addu(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source.wrapping_add(target);
        self.write_gp(dst_reg, result);
    }

    /// Add immediate unsigned
    fn addiu(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16(imm);
        let result = source.wrapping_add(imm_32);
        self.write_gp(tgt_reg, result);
    }

    /// Sub signed
    fn sub(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_sub(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(ExceptionCode::ArithmeticOverflow);
        }
    }

    /// Sub unsigned
    fn subu(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source.wrapping_sub(target);
        self.write_gp(dst_reg, result);
    }

    // Multiplication/division

    /// Multiply signed
    fn mult(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = sign_extend_32(self.read_gp(src_reg));
        let target = sign_extend_32(self.read_gp(tgt_reg));
        let result = source * target;
        self.write_hi(hi64(result as u64));
        self.write_lo(lo64(result as u64));
    }

    /// Multiply unsigned
    fn multu(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg) as u64;
        let target = self.read_gp(tgt_reg) as u64;
        let result = source * target;
        self.write_hi(hi64(result));
        self.write_lo(lo64(result));
    }

    /// Divide signed
    fn div(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg) as i32;
        let target = self.read_gp(tgt_reg) as i32;
        self.write_hi((source % target) as u32);
        self.write_lo((source / target) as u32);
    }

    /// Divide unsigned
    fn divu(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        self.write_hi(source % target);
        self.write_lo(source / target);
    }

    /// Move from hi
    fn mfhi(&mut self, dst_reg: usize) {
        self.write_gp(dst_reg, self.read_hi());
    }

    /// Move to hi
    fn mthi(&mut self, src_reg: usize) {
        self.write_hi(self.read_gp(src_reg));
    }

    /// Move from lo
    fn mflo(&mut self, dst_reg: usize) {
        self.write_gp(dst_reg, self.read_lo());
    }

    /// Move to lo
    fn mtlo(&mut self, src_reg: usize) {
        self.write_lo(self.read_gp(src_reg));
    }

    // Logic

    /// Bitwise and
    fn and(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source & target;
        self.write_gp(dst_reg, result);
    }

    /// Bitwise and immediate
    fn andi(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = imm as u32;
        let result = source & imm_32;
        self.write_gp(tgt_reg, result);
    }

    /// Bitwise or
    fn or(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source | target;
        self.write_gp(dst_reg, result);
    }

    /// Bitwise or immediate
    fn ori(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = imm as u32;
        let result = source | imm_32;
        self.write_gp(tgt_reg, result);
    }

    /// Bitwise xor
    fn xor(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source ^ target;
        self.write_gp(dst_reg, result);
    }

    /// Bitwise xor immediate
    fn xori(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = imm as u32;
        let result = source ^ imm_32;
        self.write_gp(tgt_reg, result);
    }

    /// Bitwise nor
    fn nor(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source | target;
        self.write_gp(dst_reg, !result);
    }

    // Shifts
    
    /// Shift left logical
    fn sll(&mut self, tgt_reg: usize, sh_amt: usize, dst_reg: usize) {
        let target = self.read_gp(tgt_reg);
        let result = target << sh_amt;
        self.write_gp(dst_reg, result);
    }

    /// Shift right logical
    fn srl(&mut self, tgt_reg: usize, sh_amt: usize, dst_reg: usize) {
        let target = self.read_gp(tgt_reg);
        let result = target >> sh_amt;
        self.write_gp(dst_reg, result);
    }

    /// Shift right arithmetic
    fn sra(&mut self, tgt_reg: usize, sh_amt: usize, dst_reg: usize) {
        let target = self.read_gp(tgt_reg) as i32;
        let result = target >> sh_amt;
        self.write_gp(dst_reg, result as u32);
    }

    /// Shift left logical variable
    fn sllv(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg) & 0x1F;
        let target = self.read_gp(tgt_reg);
        let result = target << source;
        self.write_gp(dst_reg, result);
    }

    /// Shift right logical variable
    fn srlv(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg) & 0x1F;
        let target = self.read_gp(tgt_reg);
        let result = target >> source;
        self.write_gp(dst_reg, result);
    }

    /// Shift right arithmetic variable
    fn srav(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg) & 0x1F;
        let target = self.read_gp(tgt_reg) as i32;
        let result = target >> source;
        self.write_gp(dst_reg, result as u32);
    }

    // Conditional sets

    /// Set on less than signed
    fn slt(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg) as i32;
        let target = self.read_gp(tgt_reg) as i32;
        let result = if source < target {1} else {0};
        self.write_gp(dst_reg, result);
    }

    /// Set on less than unsigned
    fn sltu(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = if source < target {1} else {0};
        self.write_gp(dst_reg, result);
    }

    /// Set on less than immediate signed
    fn slti(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg) as i32;
        let imm32 = sign_extend_16(imm) as i32;
        let result = if source < imm32 {1} else {0};
        self.write_gp(tgt_reg, result);
    }

    /// Set on less than immediate unsigned
    fn sltiu(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm32 = sign_extend_16(imm);
        let result = if source < imm32 {1} else {0};
        self.write_gp(tgt_reg, result);
    }

    // Memory access

    /// Load byte signed
    fn lb(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let byte = self.mem().read_byte(addr.into());
        self.write_gp(tgt_reg, sign_extend_8(byte));
    }

    /// Load byte unsigned
    fn lbu(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let byte = self.mem().read_byte(addr.into());
        self.write_gp(tgt_reg, byte as u32);
    }

    /// Load halfword signed
    fn lh(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let halfword = self.mem().read_halfword(addr.into());
        self.write_gp(tgt_reg, sign_extend_16(halfword));
    }

    /// Load halfword unsigned
    fn lhu(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let halfword = self.mem().read_halfword(addr.into());
        self.write_gp(tgt_reg, halfword as u32);
    }

    /// Load word
    fn lw(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let word = self.mem().read_word(addr.into());
        self.write_gp(tgt_reg, word);
    }

    /// Load word left
    fn lwl(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if self.mem().little_endian() { 3 - byte_addr } else { byte_addr };

        let word = self.mem().read_word(word_addr.into());
        let old_word = match byte_offset {
            0 => 0,
            1 => 0xFFFF_FFFF >> 24,
            2 => 0xFFFF_FFFF >> 16,
            3 => 0xFFFF_FFFF >> 8,
            _ => unreachable!()
        } & self.read_gp(tgt_reg);

        let shift = byte_offset * 8;

        self.write_gp(tgt_reg, old_word | (word << shift));
    }

    /// Load word right
    fn lwr(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if self.mem().little_endian() { byte_addr } else { 3 - byte_addr };

        let word = self.mem().read_word(word_addr.into());
        let old_word = match byte_offset {
            0 => 0,
            1 => 0xFFFF_FFFF << 24,
            2 => 0xFFFF_FFFF << 16,
            3 => 0xFFFF_FFFF << 8,
            _ => unreachable!()
        } & self.read_gp(tgt_reg);

        let shift = byte_offset * 8;

        self.write_gp(tgt_reg, old_word | (word >> shift));
    }

    /// Store byte
    fn sb(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.read_gp(tgt_reg) as u8;
        self.mem().write_byte(addr.into(), data);
    }

    /// Store halfword
    fn sh(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.read_gp(tgt_reg) as u16;
        self.mem().write_halfword(addr.into(), data);
    }

    /// Store word
    fn sw(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.read_gp(tgt_reg);
        self.mem().write_word(addr.into(), data);
    }

    /// Store word left
    fn swl(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if self.mem().little_endian() { 3 - byte_addr } else { byte_addr };

        let word = self.read_gp(tgt_reg);
        let old_word = match byte_offset {
            0 => 0,
            1 => 0xFFFF_FFFF << 24,
            2 => 0xFFFF_FFFF << 16,
            3 => 0xFFFF_FFFF << 8,
            _ => unreachable!()
        } & self.mem().read_word(word_addr.into());

        let shift = byte_offset * 8;

        self.mem().write_word(word_addr.into(), old_word | (word >> shift));
    }

    /// Store word right
    fn swr(&mut self, base_reg: usize, tgt_reg: usize, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if self.mem().little_endian() { byte_addr } else { 3 - byte_addr };

        let word = self.read_gp(tgt_reg);
        let old_word = match byte_offset {
            0 => 0,
            1 => 0xFFFF_FFFF >> 24,
            2 => 0xFFFF_FFFF >> 16,
            3 => 0xFFFF_FFFF >> 8,
            _ => unreachable!()
        } & self.mem().read_word(word_addr.into());

        let shift = byte_offset * 8;

        self.mem().write_word(word_addr.into(), old_word | (word << shift));
    }

    /// Load upper immediate
    fn lui(&mut self, tgt_reg: usize, imm: u16) {
        let upper_imm = (imm as u32) << 16;
        self.write_gp(tgt_reg, upper_imm);
    }

    // Branch
    
    /// Branch if equal
    fn beq(&mut self, src_reg: usize, tgt_reg: usize, offset: u16) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if source == target {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if not equal
    fn bne(&mut self, src_reg: usize, tgt_reg: usize, offset: u16) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if source != target {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if greater than zero
    fn bgtz(&mut self, src_reg: usize, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source > 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if greater than or equal to zero
    fn bgez(&mut self, src_reg: usize, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source >= 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if greater than or equal to zero and link
    fn bgezal(&mut self, src_reg: usize, offset: u16) {
        self.link_register(31);
        let source = self.read_gp(src_reg) as i32;
        if source >= 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if less than zero
    fn bltz(&mut self, src_reg: usize, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source < 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if less than or equal to zero
    fn blez(&mut self, src_reg: usize, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source <= 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if less than zero and link
    fn bltzal(&mut self, src_reg: usize, offset: u16) {
        self.link_register(31);
        let source = self.read_gp(src_reg) as i32;
        if source < 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    // Special

    /// System call
    fn syscall(&mut self) {
        self.trigger_exception(ExceptionCode::Syscall);
    }

    /// Break
    fn brk(&mut self) {
        self.trigger_exception(ExceptionCode::Breakpoint);
    }
}

impl<Mem> MIPSCore for MIPSI<Mem>
    where Mem: Mem32, <Mem as Memory>::Addr: From<u32>, MIPSI<Mem>: MIPSIInstructions<Mem> {

    fn step(&mut self) {
        let instr = self.mem.read_word(self.pc.into());
        self.pc = self.pc_next;
        self.pc_next = self.pc_next.wrapping_add(4);

        let op = || -> u8 {
            const MASK: u32 = 0xFC00_0000;
            const SHIFT: usize = 26;
            ((instr & MASK) >> SHIFT) as u8
        };
        let source = || -> usize {
            const MASK: u32 = 0x03E0_0000;
            const SHIFT: usize = 21;
            ((instr & MASK) >> SHIFT) as usize
        };
        let target = || -> usize {
            const MASK: u32 = 0x001F_0000;
            const SHIFT: usize = 16;
            ((instr & MASK) >> SHIFT) as usize
        };
        let dest = || -> usize {
            const MASK: u32 = 0x0000_F800;
            const SHIFT: usize = 11;
            ((instr & MASK) >> SHIFT) as usize
        };
        let shift_amt = || -> usize {
            const MASK: u32 = 0x0000_07C0;
            const SHIFT: usize = 6;
            ((instr & MASK) >> SHIFT) as usize
        };
        let special_op = || -> u8 {
            const MASK: u32 = 0x0000_003F;
            (instr & MASK) as u8
        };
        let imm = || -> u16 {
            instr as u16
        };

        match op() {
            0 => match special_op() {
                0x20 => self.add(source(), target(), dest()),
                0x21 => self.addu(source(), target(), dest()),
                0x22 => self.sub(source(), target(), dest()),
                0x23 => self.subu(source(), target(), dest()),

                0x18 => self.mult(source(), target()),
                0x19 => self.multu(source(), target()),
                0x1A => self.div(source(), target()),
                0x1B => self.divu(source(), target()),

                0x10 => self.mfhi(dest()),
                0x12 => self.mflo(dest()),
                0x11 => self.mthi(source()),
                0x13 => self.mthi(source()),

                0x24 => self.and(source(), target(), dest()),
                0x25 => self.or(source(), target(), dest()),
                0x26 => self.xor(source(), target(), dest()),
                0x27 => self.nor(source(), target(), dest()),

                0x00 => self.sll(target(), shift_amt(), dest()),
                0x04 => self.sllv(source(), target(), dest()),
                0x02 => self.srl(target(), shift_amt(), dest()),
                0x06 => self.srlv(source(), target(), dest()),
                0x03 => self.sra(target(), shift_amt(), dest()),
                0x07 => self.srav(source(), target(), dest()),

                0x2A => self.slt(source(), target(), dest()),
                0x2B => self.sltu(source(), target(), dest()),

                0x0C => self.syscall(),
                0x0D => self.brk(),

                _ => self.trigger_exception(ExceptionCode::ReservedInstruction),
            },
            // Immediate instructions
            0x08 => self.addi(source(), target(), imm()),
            0x09 => self.addiu(source(), target(), imm()),

            0x0C => self.andi(source(), target(), imm()),
            0x0D => self.ori(source(), target(), imm()),
            0x0E => self.xori(source(), target(), imm()),

            0x0A => self.slti(source(), target(), imm()),
            0x0B => self.sltiu(source(), target(), imm()),

            0x04 => self.beq(source(), target(), imm()),
            0x05 => self.bne(source(), target(), imm()),
            0x06 => self.blez(source(), imm()),
            0x07 => self.bgtz(source(), imm()),
            0x01 => match target() {
                0x00 => self.bltz(source(), imm()),
                0x01 => self.bgez(source(), imm()),
                0x10 => self.bltzal(source(), imm()),
                0x11 => self.bgezal(source(), imm()),
                _ => self.trigger_exception(ExceptionCode::ReservedInstruction),
            },

            0x20 => self.lb(source(), target(), imm()),
            0x24 => self.lbu(source(), target(), imm()),
            0x21 => self.lh(source(), target(), imm()),
            0x25 => self.lhu(source(), target(), imm()),
            0x23 => self.lw(source(), target(), imm()),
            0x22 => self.lwl(source(), target(), imm()),
            0x26 => self.lwr(source(), target(), imm()),

            0x28 => self.sb(source(), target(), imm()),
            0x29 => self.sh(source(), target(), imm()),
            0x2B => self.sw(source(), target(), imm()),
            0x2A => self.swl(source(), target(), imm()),
            0x2E => self.swr(source(), target(), imm()),

            0x0F => self.lui(target(), imm()),
            _ => self.trigger_exception(ExceptionCode::ReservedInstruction),
        }
    }
}