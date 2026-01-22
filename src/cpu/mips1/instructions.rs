use super::*;

/// The set of instructions defined in MIPS I.
/// 
/// The arguments must have been decoded prior to calling these.
/// If a register number argument has a value greater than 31, the result is undefined.
pub trait MIPSIInstructions<Mem>: MIPSICore<Mem = Mem>
    where Mem: Mem32, Mem::Addr: From<u32> {
    // Arithmetic

    /// Add signed
    fn add(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_add(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(ExceptionCode::ArithmeticOverflow);
        }
    }

    /// Add immediate signed
    fn addi(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16(imm);
        if let Some(result) = source.checked_add(imm_32) {
            self.write_gp(tgt_reg, result);
        } else {
            self.trigger_exception(ExceptionCode::ArithmeticOverflow);
        }
    }

    /// Add unsigned
    fn addu(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source.wrapping_add(target);
        self.write_gp(dst_reg, result);
    }

    /// Add immediate unsigned
    fn addiu(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16(imm);
        let result = source.wrapping_add(imm_32);
        self.write_gp(tgt_reg, result);
    }

    /// Sub signed
    fn sub(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_sub(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(ExceptionCode::ArithmeticOverflow);
        }
    }

    /// Sub unsigned
    fn subu(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source.wrapping_sub(target);
        self.write_gp(dst_reg, result);
    }

    // Multiplication/division

    /// Multiply signed
    fn mult(&mut self, src_reg: u8, tgt_reg: u8) {
        let source = sign_extend_32(self.read_gp(src_reg));
        let target = sign_extend_32(self.read_gp(tgt_reg));
        let result = source * target;
        self.write_hi(hi64(result as u64));
        self.write_lo(lo64(result as u64));
    }

    /// Multiply unsigned
    fn multu(&mut self, src_reg: u8, tgt_reg: u8) {
        let source = self.read_gp(src_reg) as u64;
        let target = self.read_gp(tgt_reg) as u64;
        let result = source * target;
        self.write_hi(hi64(result));
        self.write_lo(lo64(result));
    }

    /// Divide signed
    fn div(&mut self, src_reg: u8, tgt_reg: u8) {
        let source = self.read_gp(src_reg) as i32;
        let target = self.read_gp(tgt_reg) as i32;
        self.write_hi((source % target) as u32);
        self.write_lo((source / target) as u32);
    }

    /// Divide unsigned
    fn divu(&mut self, src_reg: u8, tgt_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        self.write_hi(source % target);
        self.write_lo(source / target);
    }

    /// Move from hi
    fn mfhi(&mut self, dst_reg: u8) {
        self.write_gp(dst_reg, self.read_hi());
    }

    /// Move to hi
    fn mthi(&mut self, src_reg: u8) {
        self.write_hi(self.read_gp(src_reg));
    }

    /// Move from lo
    fn mflo(&mut self, dst_reg: u8) {
        self.write_gp(dst_reg, self.read_lo());
    }

    /// Move to lo
    fn mtlo(&mut self, src_reg: u8) {
        self.write_lo(self.read_gp(src_reg));
    }

    // Logic

    /// Bitwise and
    fn and(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source & target;
        self.write_gp(dst_reg, result);
    }

    /// Bitwise and immediate
    fn andi(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = imm as u32;
        let result = source & imm_32;
        self.write_gp(tgt_reg, result);
    }

    /// Bitwise or
    fn or(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source | target;
        self.write_gp(dst_reg, result);
    }

    /// Bitwise or immediate
    fn ori(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = imm as u32;
        let result = source | imm_32;
        self.write_gp(tgt_reg, result);
    }

    /// Bitwise xor
    fn xor(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source ^ target;
        self.write_gp(dst_reg, result);
    }

    /// Bitwise xor immediate
    fn xori(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = imm as u32;
        let result = source ^ imm_32;
        self.write_gp(tgt_reg, result);
    }

    /// Bitwise nor
    fn nor(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source | target;
        self.write_gp(dst_reg, !result);
    }

    // Shifts
    
    /// Shift left logical
    fn sll(&mut self, tgt_reg: u8, sh_amt: u8, dst_reg: u8) {
        let target = self.read_gp(tgt_reg);
        let result = target << sh_amt;
        self.write_gp(dst_reg, result);
    }

    /// Shift right logical
    fn srl(&mut self, tgt_reg: u8, sh_amt: u8, dst_reg: u8) {
        let target = self.read_gp(tgt_reg);
        let result = target >> sh_amt;
        self.write_gp(dst_reg, result);
    }

    /// Shift right arithmetic
    fn sra(&mut self, tgt_reg: u8, sh_amt: u8, dst_reg: u8) {
        let target = self.read_gp(tgt_reg) as i32;
        let result = target >> sh_amt;
        self.write_gp(dst_reg, result as u32);
    }

    /// Shift left logical variable
    fn sllv(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg) & 0x1F;
        let target = self.read_gp(tgt_reg);
        let result = target << source;
        self.write_gp(dst_reg, result);
    }

    /// Shift right logical variable
    fn srlv(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg) & 0x1F;
        let target = self.read_gp(tgt_reg);
        let result = target >> source;
        self.write_gp(dst_reg, result);
    }

    /// Shift right arithmetic variable
    fn srav(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg) & 0x1F;
        let target = self.read_gp(tgt_reg) as i32;
        let result = target >> source;
        self.write_gp(dst_reg, result as u32);
    }

    // Conditional sets

    /// Set on less than signed
    fn slt(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg) as i32;
        let target = self.read_gp(tgt_reg) as i32;
        let result = if source < target {1} else {0};
        self.write_gp(dst_reg, result);
    }

    /// Set on less than unsigned
    fn sltu(&mut self, src_reg: u8, tgt_reg: u8, dst_reg: u8) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = if source < target {1} else {0};
        self.write_gp(dst_reg, result);
    }

    /// Set on less than immediate signed
    fn slti(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg) as i32;
        let imm32 = sign_extend_16(imm) as i32;
        let result = if source < imm32 {1} else {0};
        self.write_gp(tgt_reg, result);
    }

    /// Set on less than immediate unsigned
    fn sltiu(&mut self, src_reg: u8, tgt_reg: u8, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm32 = sign_extend_16(imm);
        let result = if source < imm32 {1} else {0};
        self.write_gp(tgt_reg, result);
    }

    // Memory access

    /// Load byte signed
    fn lb(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let byte = self.mut_mem().read_byte(addr.into());
        self.write_gp(tgt_reg, sign_extend_8(byte));
    }

    /// Load byte unsigned
    fn lbu(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let byte = self.mut_mem().read_byte(addr.into());
        self.write_gp(tgt_reg, byte as u32);
    }

    /// Load halfword signed
    fn lh(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let halfword = self.mut_mem().read_halfword(addr.into());
        self.write_gp(tgt_reg, sign_extend_16(halfword));
    }

    /// Load halfword unsigned
    fn lhu(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let halfword = self.mut_mem().read_halfword(addr.into());
        self.write_gp(tgt_reg, halfword as u32);
    }

    /// Load word
    fn lw(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let word = self.mut_mem().read_word(addr.into());
        self.write_gp(tgt_reg, word);
    }

    /// Load word left
    fn lwl(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if Mem::LITTLE_ENDIAN { 3 - byte_addr } else { byte_addr };

        let word = self.mut_mem().read_word(word_addr.into());
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
    fn lwr(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if Mem::LITTLE_ENDIAN { byte_addr } else { 3 - byte_addr };

        let word = self.mut_mem().read_word(word_addr.into());
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
    fn sb(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.read_gp(tgt_reg) as u8;
        self.mut_mem().write_byte(addr.into(), data);
    }

    /// Store halfword
    fn sh(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.read_gp(tgt_reg) as u16;
        self.mut_mem().write_halfword(addr.into(), data);
    }

    /// Store word
    fn sw(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.read_gp(tgt_reg);
        self.mut_mem().write_word(addr.into(), data);
    }

    /// Store word left
    fn swl(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if Mem::LITTLE_ENDIAN { 3 - byte_addr } else { byte_addr };

        let word = self.read_gp(tgt_reg);
        let old_word = match byte_offset {
            0 => 0,
            1 => 0xFFFF_FFFF << 24,
            2 => 0xFFFF_FFFF << 16,
            3 => 0xFFFF_FFFF << 8,
            _ => unreachable!()
        } & self.mut_mem().read_word(word_addr.into());

        let shift = byte_offset * 8;

        self.mut_mem().write_word(word_addr.into(), old_word | (word >> shift));
    }

    /// Store word right
    fn swr(&mut self, base_reg: u8, tgt_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);

        let word_addr = addr & 0xFFFF_FFFC;
        let byte_addr = addr & 3;
        let byte_offset = if Mem::LITTLE_ENDIAN { byte_addr } else { 3 - byte_addr };

        let word = self.read_gp(tgt_reg);
        let old_word = match byte_offset {
            0 => 0,
            1 => 0xFFFF_FFFF >> 24,
            2 => 0xFFFF_FFFF >> 16,
            3 => 0xFFFF_FFFF >> 8,
            _ => unreachable!()
        } & self.mut_mem().read_word(word_addr.into());

        let shift = byte_offset * 8;

        self.mut_mem().write_word(word_addr.into(), old_word | (word << shift));
    }

    /// Load upper immediate
    fn lui(&mut self, tgt_reg: u8, imm: u16) {
        let upper_imm = (imm as u32) << 16;
        self.write_gp(tgt_reg, upper_imm);
    }

    // Branch
    
    /// Branch if equal
    fn beq(&mut self, src_reg: u8, tgt_reg: u8, offset: u16) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if source == target {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if not equal
    fn bne(&mut self, src_reg: u8, tgt_reg: u8, offset: u16) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if source != target {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if greater than zero
    fn bgtz(&mut self, src_reg: u8, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source > 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if greater than or equal to zero
    fn bgez(&mut self, src_reg: u8, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source >= 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if greater than or equal to zero and link
    fn bgezal(&mut self, src_reg: u8, offset: u16) {
        self.link_register(31);
        let source = self.read_gp(src_reg) as i32;
        if source >= 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if less than zero
    fn bltz(&mut self, src_reg: u8, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source < 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if less than or equal to zero
    fn blez(&mut self, src_reg: u8, offset: u16) {
        let source = self.read_gp(src_reg) as i32;
        if source <= 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    /// Branch if less than zero and link
    fn bltzal(&mut self, src_reg: u8, offset: u16) {
        self.link_register(31);
        let source = self.read_gp(src_reg) as i32;
        if source < 0 {
            let offset32 = sign_extend_16(offset) << 2;
            self.branch(offset32);
        }
    }

    // Jump

    /// Jump
    fn j(&mut self, target: u32) {
        self.jump(target << 2);
    }

    /// Jump and link
    fn jal(&mut self, target: u32) {
        self.link_register(31);
        self.jump(target << 2);
    }

    /// Jump register
    fn jr(&mut self, src_reg: u8) {
        let dest = self.read_gp(src_reg);
        self.jump(dest);
    }

    /// Jump and link register
    fn jalr(&mut self, src_reg: u8, dst_reg: u8) {
        self.link_register(dst_reg);
        let dest = self.read_gp(src_reg);
        self.jump(dest);
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

    // Coprocessor

    /// Move register to coprocessor
    fn mtcz(&mut self, coproc: Coproc, tgt_reg: u8, cop_reg: u8) {
        let val = self.read_gp(tgt_reg);
        match coproc {
            Coproc::_0 => self.coproc_0().move_to_reg(cop_reg, val),
            Coproc::_1 => if let Some(cop) = self.coproc_1() {cop.move_to_reg(cop_reg, val)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_2 => if let Some(cop) = self.coproc_2() {cop.move_to_reg(cop_reg, val)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_3 => if let Some(cop) = self.coproc_3() {cop.move_to_reg(cop_reg, val)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
        }
    }

    /// Move register from coprocessor
    fn mfcz(&mut self, coproc: Coproc, tgt_reg: u8, cop_reg: u8) {
        if let Some(val) = match coproc {
            Coproc::_0 => Some(self.coproc_0().move_from_reg(cop_reg)),
            Coproc::_1 => self.coproc_1().map(|cop| cop.move_from_reg(cop_reg)),
            Coproc::_2 => self.coproc_2().map(|cop| cop.move_from_reg(cop_reg)),
            Coproc::_3 => self.coproc_3().map(|cop| cop.move_from_reg(cop_reg))
        } {
            self.write_gp(tgt_reg, val);
        } else {
            self.trigger_exception(ExceptionCode::CoProcUnusable);
        }
    }

    /// Move control to coprocessor
    fn ctcz(&mut self, coproc: Coproc, tgt_reg: u8, ctrl_reg: u8) {
        let val = self.read_gp(tgt_reg);
        match coproc {
            Coproc::_0 => unreachable!(),
            Coproc::_1 => if let Some(cop) = self.coproc_1() {cop.move_to_control(ctrl_reg, val)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_2 => if let Some(cop) = self.coproc_2() {cop.move_to_control(ctrl_reg, val)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_3 => if let Some(cop) = self.coproc_3() {cop.move_to_control(ctrl_reg, val)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
        }
    }

    /// Move control from coprocessor
    fn cfcz(&mut self, coproc: Coproc, tgt_reg: u8, ctrl_reg: u8) {
        if let Some(val) = match coproc {
            Coproc::_0 => unreachable!(),
            Coproc::_1 => self.coproc_1().map(|cop| cop.move_from_control(ctrl_reg)),
            Coproc::_2 => self.coproc_2().map(|cop| cop.move_from_control(ctrl_reg)),
            Coproc::_3 => self.coproc_3().map(|cop| cop.move_from_control(ctrl_reg))
        } {
            self.write_gp(tgt_reg, val);
        } else {
            self.trigger_exception(ExceptionCode::CoProcUnusable);
        }
    }

    /// Load word into coprocessor
    fn lwcz(&mut self, coproc: Coproc, base_reg: u8, cop_reg: u8, offset: u16) {
        let base = self.read_gp(base_reg);
        let offset32 = sign_extend_16(offset);
        let addr = base.wrapping_add(offset32);
        let data = self.mut_mem().read_word(addr.into());
        match coproc {
            Coproc::_0 => unreachable!(),
            Coproc::_1 => if let Some(cop) = self.coproc_1() {cop.move_to_reg(cop_reg, data)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_2 => if let Some(cop) = self.coproc_2() {cop.move_to_reg(cop_reg, data)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_3 => if let Some(cop) = self.coproc_3() {cop.move_to_reg(cop_reg, data)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
        }
    }

    /// Store word from coprocessor
    fn swcz(&mut self, coproc: Coproc, base_reg: u8, cop_reg: u8, offset: u16) {
        if let Some(data) = match coproc {
            Coproc::_0 => unreachable!(),
            Coproc::_1 => self.coproc_1().map(|cop| cop.move_from_reg(cop_reg)),
            Coproc::_2 => self.coproc_2().map(|cop| cop.move_from_reg(cop_reg)),
            Coproc::_3 => self.coproc_3().map(|cop| cop.move_from_reg(cop_reg))
        } {
            let base = self.read_gp(base_reg);
            let offset32 = sign_extend_16(offset);
            let addr = base.wrapping_add(offset32);
            self.mut_mem().write_word(addr.into(), data);
        } else {
            self.trigger_exception(ExceptionCode::CoProcUnusable);
        }
    }

    /// Coprocessor operation
    fn copz(&mut self, coproc: Coproc, cofun: u32) {
        match coproc {
            Coproc::_0 => self.coproc_0().operation(cofun),
            Coproc::_1 => if let Some(cop) = self.coproc_1() {cop.operation(cofun)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_2 => if let Some(cop) = self.coproc_2() {cop.operation(cofun)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
            Coproc::_3 => if let Some(cop) = self.coproc_3() {cop.operation(cofun)} else {self.trigger_exception(ExceptionCode::CoProcUnusable)},
        }
    }
}

impl<
    Mem: Mem32,
    C0: Coprocessor0,
    C1: Coprocessor,
    C2: Coprocessor,
    C3: Coprocessor
> MIPSCore for MIPSI<Mem, C0, C1, C2, C3>
    where Mem::Addr: From<u32>, MIPSI<Mem, C0, C1, C2, C3>: MIPSIInstructions<Mem> {

    fn step(&mut self) {
        use MIPSIInstruction::*;

        self.current_instr_addr = self.pc;
        let instr_bits = self.mem.read_word(self.current_instr_addr.into());
        self.pc = self.pc_next;
        self.pc_next = self.pc_next.wrapping_add(4);

        if let Some(instr) = MIPSIInstruction::decode(instr_bits) {
            match instr {
                ADD{source, target, dest}   => self.add(source, target, dest),
                ADDU{source, target, dest}  => self.addu(source, target, dest),
                SUB{source, target, dest}   => self.sub(source, target, dest),
                SUBU{source, target, dest}  => self.subu(source, target, dest),
                MULT{source, target}        => self.mult(source, target),
                MULTU{source, target}       => self.multu(source, target),
                DIV{source, target}         => self.div(source, target),
                DIVU{source, target}        => self.divu(source, target),
                MFHI{dest}                  => self.mfhi(dest),
                MFLO{dest}                  => self.mflo(dest), 
                MTHI{source}                => self.mthi(source),
                MTLO{source}                => self.mtlo(source),
                AND{source, target, dest}   => self.and(source, target, dest),
                OR{source, target, dest}    => self.or(source, target, dest),
                XOR{source, target, dest}   => self.xor(source, target, dest),
                NOR{source, target, dest}   => self.nor(source, target, dest),

                SLL{target, shift_amt, dest}    => self.sll(target, shift_amt, dest),
                SLLV{source, target, dest}      => self.sllv(source, target, dest),
                SRL{target, shift_amt, dest}    => self.srl(target, shift_amt, dest),
                SRLV{source, target, dest}      => self.srlv(source, target, dest),
                SRA{target, shift_amt, dest}    => self.sra(target, shift_amt, dest),
                SRAV{source, target, dest}      => self.srav(source, target, dest),

                SLT{source, target, dest}   => self.slt(source, target, dest),
                SLTU{source, target, dest}  => self.sltu(source, target, dest),

                JR{source}          => self.jr(source),
                JALR{source, dest}  => self.jalr(source, dest),
                SYSCALL             => self.syscall(),
                BRK                 => self.brk(),

                ADDI{source, target, imm}   => self.addi(source, target, imm),
                ADDIU{source, target, imm}  => self.addiu(source, target, imm),
                ANDI{source, target, imm}   => self.andi(source, target, imm),
                ORI{source, target, imm}    => self.ori(source, target, imm),
                XORI{source, target, imm}   => self.xori(source, target, imm),
                SLTI{source, target, imm}   => self.slti(source, target, imm),
                SLTIU{source, target, imm}  => self.sltiu(source, target, imm),

                BEQ{source, target, imm}    => self.beq(source, target, imm),
                BNE{source, target, imm}    => self.bne(source, target, imm),
                BLEZ{source, imm}           => self.blez(source, imm),
                BGTZ{source, imm}           => self.bgtz(source, imm),
                BLTZ{source, imm}           => self.bltz(source, imm),
                BGEZ{source, imm}           => self.bgez(source, imm),
                BLTZAL{source, imm}         => self.bltzal(source, imm),
                BGEZAL{source, imm}         => self.bgezal(source, imm),

                LB{source, target, imm}     => self.lb(source, target, imm),
                LBU{source, target, imm}    => self.lbu(source, target, imm),
                LH{source, target, imm}     => self.lh(source, target, imm),
                LHU{source, target, imm}    => self.lhu(source, target, imm),
                LW{source, target, imm}     => self.lw(source, target, imm),
                LWL{source, target, imm}    => self.lwl(source, target, imm),
                LWR{source, target, imm}    => self.lwr(source, target, imm),

                SB{source, target, imm}     => self.sb(source, target, imm),
                SH{source, target, imm}     => self.sh(source, target, imm),
                SW{source, target, imm}     => self.sw(source, target, imm),
                SWL{source, target, imm}    => self.swl(source, target, imm),
                SWR{source, target, imm}    => self.swr(source, target, imm),

                LUI{target, imm} => self.lui(target, imm),

                J{addr}     => self.j(addr),
                JAL{addr}   => self.jal(addr),

                MFCZ{coproc, target, dest}          => self.mfcz(coproc, target, dest),
                MTCZ{coproc, target, dest}          => self.mtcz(coproc, target, dest),
                CFCZ{coproc, target, dest}          => self.cfcz(coproc, target, dest),
                CTCZ{coproc, target, dest}          => self.ctcz(coproc, target, dest),
                COPZ{coproc, fun}                   => self.copz(coproc, fun),
                LWCZ{coproc, source, target, imm}   => self.lwcz(coproc, source, target, imm),
                SWCZ{coproc, source, target, imm}   => self.swcz(coproc, source, target, imm),
            }
        } else {
            self.trigger_exception(ExceptionCode::ReservedInstruction);
        }

        let cycles = 1; // TODO: count cycles
        let int = self.mem.clock(cycles);
        if self.coproc0.external_interrupt(int) {
            self.trigger_exception(ExceptionCode::Interrupt);
        }
    }

    fn reset(&mut self) {
        let addr = self.coproc0.reset();
        self.pc = addr;
        self.pc_next = addr.wrapping_add(4);
    }
}

#[derive(Clone)]
pub enum MIPSIInstruction {
    ADD{source: u8, target: u8, dest: u8},
    ADDU{source: u8, target: u8, dest: u8},
    SUB{source: u8, target: u8, dest: u8},
    SUBU{source: u8, target: u8, dest: u8},
    MULT{source: u8, target: u8},
    MULTU{source: u8, target: u8},
    DIV{source: u8, target: u8},
    DIVU{source: u8, target: u8},
    MFHI{dest: u8},
    MFLO{dest: u8},
    MTHI{source: u8},
    MTLO{source: u8},
    AND{source: u8, target: u8, dest: u8},
    OR{source: u8, target: u8, dest: u8},
    XOR{source: u8, target: u8, dest: u8},
    NOR{source: u8, target: u8, dest: u8},

    SLL{target: u8, shift_amt: u8, dest: u8},
    SLLV{source: u8, target: u8, dest: u8},
    SRL{target: u8, shift_amt: u8, dest: u8},
    SRLV{source: u8, target: u8, dest: u8},
    SRA{target: u8, shift_amt: u8, dest: u8},
    SRAV{source: u8, target: u8, dest: u8},

    SLT{source: u8, target: u8, dest: u8},
    SLTU{source: u8, target: u8, dest: u8},

    JR{source: u8},
    JALR{source: u8, dest: u8},
    SYSCALL,
    BRK,

    ADDI{source: u8, target: u8, imm: u16},
    ADDIU{source: u8, target: u8, imm: u16},
    ANDI{source: u8, target: u8, imm: u16},
    ORI{source: u8, target: u8, imm: u16},
    XORI{source: u8, target: u8, imm: u16},
    SLTI{source: u8, target: u8, imm: u16},
    SLTIU{source: u8, target: u8, imm: u16},

    BEQ{source: u8, target: u8, imm: u16},
    BNE{source: u8, target: u8, imm: u16},
    BLEZ{source: u8, imm: u16},
    BGTZ{source: u8, imm: u16},
    BLTZ{source: u8, imm: u16},
    BGEZ{source: u8, imm: u16},
    BLTZAL{source: u8, imm: u16},
    BGEZAL{source: u8, imm: u16},

    LB{source: u8, target: u8, imm: u16},
    LBU{source: u8, target: u8, imm: u16},
    LH{source: u8, target: u8, imm: u16},
    LHU{source: u8, target: u8, imm: u16},
    LW{source: u8, target: u8, imm: u16},
    LWL{source: u8, target: u8, imm: u16},
    LWR{source: u8, target: u8, imm: u16},

    SB{source: u8, target: u8, imm: u16},
    SH{source: u8, target: u8, imm: u16},
    SW{source: u8, target: u8, imm: u16},
    SWL{source: u8, target: u8, imm: u16},
    SWR{source: u8, target: u8, imm: u16},

    LUI{target: u8, imm: u16},

    J{addr: u32},
    JAL{addr: u32},

    MFCZ{coproc: Coproc, target: u8, dest: u8},
    MTCZ{coproc: Coproc, target: u8, dest: u8},
    CFCZ{coproc: Coproc, target: u8, dest: u8},
    CTCZ{coproc: Coproc, target: u8, dest: u8},
    COPZ{coproc: Coproc, fun: u32},
    LWCZ{coproc: Coproc, source: u8, target: u8, imm: u16},
    SWCZ{coproc: Coproc, source: u8, target: u8, imm: u16},
}

impl MIPSIInstruction {
    pub fn decode(instr: u32) -> Option<Self> {
        use MIPSIInstruction::*;
        let op = || -> u8 {
            const MASK: u32 = 0xFC00_0000;
            const SHIFT: usize = 26;
            ((instr & MASK) >> SHIFT) as u8
        };
        let source = {
            const MASK: u32 = 0x03E0_0000;
            const SHIFT: usize = 21;
            ((instr & MASK) >> SHIFT) as u8
        };
        let target = {
            const MASK: u32 = 0x001F_0000;
            const SHIFT: usize = 16;
            ((instr & MASK) >> SHIFT) as u8
        };
        let dest = {
            const MASK: u32 = 0x0000_F800;
            const SHIFT: usize = 11;
            ((instr & MASK) >> SHIFT) as u8
        };
        let shift_amt = {
            const MASK: u32 = 0x0000_07C0;
            const SHIFT: usize = 6;
            ((instr & MASK) >> SHIFT) as u8
        };
        let special_op = {
            const MASK: u32 = 0x0000_003F;
            (instr & MASK) as u8
        };
        let imm = {
            instr as u16
        };
        let jump_target = {
            const MASK: u32 = 0x03FF_FFFF;
            instr & MASK
        };
        let cofun = {
            const MASK: u32 = 0x01FF_FFFF;
            instr & MASK
        };
        match op() {
            0 => match special_op {
                0x20 => Some(ADD{source, target, dest}),
                0x21 => Some(ADDU{source, target, dest}),
                0x22 => Some(SUB{source, target, dest}),
                0x23 => Some(SUBU{source, target, dest}),

                0x18 => Some(MULT{source, target}),
                0x19 => Some(MULTU{source, target}),
                0x1A => Some(DIV{source, target}),
                0x1B => Some(DIVU{source, target}),

                0x10 => Some(MFHI{dest}),
                0x12 => Some(MFLO{dest}),
                0x11 => Some(MTHI{source}),
                0x13 => Some(MTLO{source}),

                0x24 => Some(AND{source, target, dest}),
                0x25 => Some(OR{source, target, dest}),
                0x26 => Some(XOR{source, target, dest}),
                0x27 => Some(NOR{source, target, dest}),

                0x00 => Some(SLL{target, shift_amt, dest}),
                0x04 => Some(SLLV{source, target, dest}),
                0x02 => Some(SRL{target, shift_amt, dest}),
                0x06 => Some(SRLV{source, target, dest}),
                0x03 => Some(SRA{target, shift_amt, dest}),
                0x07 => Some(SRAV{source, target, dest}),

                0x2A => Some(SLT{source, target, dest}),
                0x2B => Some(SLTU{source, target, dest}),

                0x08 => Some(JR{source}),
                0x09 => Some(JALR{source, dest}),

                0x0C => Some(SYSCALL),
                0x0D => Some(BRK),

                _ => None,
            },
            // Immediate instructions
            0x08 => Some(ADDI{source, target, imm}),
            0x09 => Some(ADDIU{source, target, imm}),

            0x0C => Some(ANDI{source, target, imm}),
            0x0D => Some(ORI{source, target, imm}),
            0x0E => Some(XORI{source, target, imm}),

            0x0A => Some(SLTI{source, target, imm}),
            0x0B => Some(SLTIU{source, target, imm}),

            0x04 => Some(BEQ{source, target, imm}),
            0x05 => Some(BNE{source, target, imm}),
            0x06 => Some(BLEZ{source, imm}),
            0x07 => Some(BGTZ{source, imm}),
            0x01 => match target {
                0x00 => Some(BLTZ{source, imm}),
                0x01 => Some(BGEZ{source, imm}),
                0x10 => Some(BLTZAL{source, imm}),
                0x11 => Some(BGEZAL{source, imm}),
                _ => None,
            },

            0x20 => Some(LB{source, target, imm}),
            0x24 => Some(LBU{source, target, imm}),
            0x21 => Some(LH{source, target, imm}),
            0x25 => Some(LHU{source, target, imm}),
            0x23 => Some(LW{source, target, imm}),
            0x22 => Some(LWL{source, target, imm}),
            0x26 => Some(LWR{source, target, imm}),

            0x28 => Some(SB{source, target, imm}),
            0x29 => Some(SH{source, target, imm}),
            0x2B => Some(SW{source, target, imm}),
            0x2A => Some(SWL{source, target, imm}),
            0x2E => Some(SWR{source, target, imm}),

            0x0F => Some(LUI{target, imm}),

            // Jump instructions
            0x02 => Some(J{addr: jump_target}),
            0x03 => Some(JAL{addr: jump_target}),

            // Coprocessor
            0x10 => match source {
                0x00 => Some(MFCZ{coproc: Coproc::_0, target, dest}),
                0x04 => Some(MTCZ{coproc: Coproc::_0, target, dest}),
                x if (x & 0x10) == 0x10 => Some(COPZ{coproc: Coproc::_0, fun: cofun}),
                _ => None,
            },
            0x11 => match source {
                0x00 => Some(MFCZ{coproc: Coproc::_1, target, dest}),
                0x02 => Some(CFCZ{coproc: Coproc::_1, target, dest}),
                0x04 => Some(MTCZ{coproc: Coproc::_1, target, dest}),
                0x06 => Some(CTCZ{coproc: Coproc::_1, target, dest}),
                x if (x & 0x10) == 0x10 => Some(COPZ{coproc: Coproc::_1, fun: cofun}),
                _ => None,
            },
            0x12 => match source {
                0x00 => Some(MFCZ{coproc: Coproc::_2, target, dest}),
                0x02 => Some(CFCZ{coproc: Coproc::_2, target, dest}),
                0x04 => Some(MTCZ{coproc: Coproc::_2, target, dest}),
                0x06 => Some(CTCZ{coproc: Coproc::_2, target, dest}),
                x if (x & 0x10) == 0x10 => Some(COPZ{coproc: Coproc::_2, fun: cofun}),
                _ => None,
            },
            0x13 => match source {
                0x00 => Some(MFCZ{coproc: Coproc::_3, target, dest}),
                0x02 => Some(CFCZ{coproc: Coproc::_3, target, dest}),
                0x04 => Some(MTCZ{coproc: Coproc::_3, target, dest}),
                0x06 => Some(CTCZ{coproc: Coproc::_3, target, dest}),
                x if (x & 0x10) == 0x10 => Some(COPZ{coproc: Coproc::_3, fun: cofun}),
                _ => None,
            },
            0x31 => Some(LWCZ{coproc: Coproc::_1, source, target, imm}),
            0x32 => Some(LWCZ{coproc: Coproc::_2, source, target, imm}),
            0x33 => Some(LWCZ{coproc: Coproc::_3, source, target, imm}),

            0x39 => Some(SWCZ{coproc: Coproc::_1, source, target, imm}),
            0x3A => Some(SWCZ{coproc: Coproc::_2, source, target, imm}),
            0x3B => Some(SWCZ{coproc: Coproc::_3, source, target, imm}),

            _ => None,
        }
    }
}

impl std::fmt::Display for MIPSIInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MIPSIInstruction::*;
        match *self {
            ADD{source, target, dest} => write!(f, "ADD {},{},{}", dest, source, target),
            ADDU{source, target, dest} => write!(f, "ADDU {},{},{}", dest, source, target),
            SUB{source, target, dest} => write!(f, "SUB {},{},{}", dest, source, target),
            SUBU{source, target, dest} => write!(f, "SUBU {},{},{}", dest, source, target),
            MULT{source, target} => write!(f, "MULT {},{}", source, target),
            MULTU{source, target} => write!(f, "MULTU {},{}", source, target),
            DIV{source, target} => write!(f, "DIV {},{}", source, target),
            DIVU{source, target} => write!(f, "DIVU {},{}", source, target),
            MFHI{dest} => write!(f, "MFHI {}", dest),
            MFLO{dest} => write!(f, "MFLO {}", dest),
            MTHI{source} => write!(f, "MTHI {}", source),
            MTLO{source} => write!(f, "MTLO {}", source),
            AND{source, target, dest} => write!(f, "AND {},{},{}", dest, source, target),
            OR{source, target, dest} => write!(f, "OR {},{},{}", dest, source, target),
            XOR{source, target, dest} => write!(f, "XOR {},{},{}", dest, source, target),
            NOR{source, target, dest} => write!(f, "NOR {},{},{}", dest, source, target),

            SLL{target, shift_amt, dest} => write!(f, "SLL {},{},{}", dest, target, shift_amt),
            SLLV{source, target, dest} => write!(f, "SLLV {},{},{}", dest, source, target),
            SRL{target, shift_amt, dest} => write!(f, "SRL {},{},{}", dest, target, shift_amt),
            SRLV{source, target, dest} => write!(f, "SRLV {},{},{}", dest, source, target),
            SRA{target, shift_amt, dest} => write!(f, "SRA {},{},{}", dest, target, shift_amt),
            SRAV{source, target, dest} => write!(f, "SRAV {},{},{}", dest, source, target),

            SLT{source, target, dest} => write!(f, "SLT {},{},{}", dest, source, target),
            SLTU{source, target, dest} => write!(f, "SLTU {},{},{}", dest, source, target),

            JR{source} => write!(f, "JR {}", source),
            JALR{source, dest} => write!(f, "JALR {}(, {})", source, dest),
            SYSCALL => write!(f, "SYSCALL"),
            BRK => write!(f, "BRK"),

            ADDI{source, target, imm} => write!(f, "ADDI {},{},${:X}", target, source, imm),
            ADDIU{source, target, imm} => write!(f, "ADDIU {},{},${:X}", target, source, imm),
            ANDI{source, target, imm} => write!(f, "ANDI {},{},${:X}", target, source, imm),
            ORI{source, target, imm} => write!(f, "ORI {},{},${:X}", target, source, imm),
            XORI{source, target, imm} => write!(f, "XORI {},{},${:X}", target, source, imm),
            SLTI{source, target, imm} => write!(f, "SLTI {},{},${:X}", target, source, imm),
            SLTIU{source, target, imm} => write!(f, "SLTIU {},{},${:X}", target, source, imm),

            BEQ{source, target, imm} => write!(f, "BEQ {},{},${:X}", source, target, imm),
            BNE{source, target, imm} => write!(f, "BNE {},{},${:X}", source, target, imm),
            BLEZ{source, imm} => write!(f, "BLEZ {},${:X}", source, imm),
            BGTZ{source, imm} => write!(f, "BGTZ {},${:X}", source, imm),
            BLTZ{source, imm} => write!(f, "BLTZ {},${:X}", source, imm),
            BGEZ{source, imm} => write!(f, "BGEZ {},${:X}", source, imm),
            BLTZAL{source, imm} => write!(f, "BLTZAL {},${:X}", source, imm),
            BGEZAL{source, imm} => write!(f, "BGEZAL {},${:X}", source, imm),

            LB{source, target, imm} => write!(f, "LB {},{}+${:X}", target, source, imm),
            LBU{source, target, imm} => write!(f, "LBU {},{}+${:X}", target, source, imm),
            LH{source, target, imm} => write!(f, "LH {},{}+${:X}", target, source, imm),
            LHU{source, target, imm} => write!(f, "LHU {},{}+${:X}", target, source, imm),
            LW{source, target, imm} => write!(f, "LW {},{}+${:X}", target, source, imm),
            LWL{source, target, imm} => write!(f, "LWL {},{}+${:X}", target, source, imm),
            LWR{source, target, imm} => write!(f, "LWR {},{}+${:X}", target, source, imm),

            SB{source, target, imm} => write!(f, "SB {},{}+${:X}", target, source, imm),
            SH{source, target, imm} => write!(f, "SH {},{}+${:X}", target, source, imm),
            SW{source, target, imm} => write!(f, "SW {},{}+${:X}", target, source, imm),
            SWL{source, target, imm} => write!(f, "SWL {},{}+${:X}", target, source, imm),
            SWR{source, target, imm} => write!(f, "SWR {},{}+${:X}", target, source, imm),

            LUI{target, imm} => write!(f, "LUI {},${:X}", target, imm),

            J{addr} => write!(f, "J ${:X}", addr << 2),
            JAL{addr} => write!(f, "JAL ${:X}", addr << 2),

            MFCZ{coproc, target, dest} => write!(f, "MFC{} {},{}", coproc as usize, dest, target),
            MTCZ{coproc, target, dest} => write!(f, "MTC{} {},{}", coproc as usize, target, dest),
            CFCZ{coproc, target, dest} => write!(f, "CFC{} {},{}", coproc as usize, dest, target),
            CTCZ{coproc, target, dest} => write!(f, "CTC{} {},{}", coproc as usize, target, dest),
            COPZ{coproc, fun} => write!(f, "COP{} {}", coproc as usize, fun),
            LWCZ{coproc, source, target, imm} => write!(f, "LWC{} {},{}+${:X}", coproc as usize, target, source, imm),
            SWCZ{coproc, source, target, imm} => write!(f, "SWC{} {},{}+${:X}", coproc as usize, target, source, imm),
        }
    }
}