use super::*;

/// Mips I processor.
#[derive(Default)]
pub struct MIPSI {
    gp_reg:     [u32; 32],
    hi:         u32,
    lo:         u32,

    pc:         u32,
    pc_next:    u32,
}

impl MIPSICore for MIPSI {
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

    fn trigger_exception(&mut self, exception: Exception) {

    }
}

impl MIPSIInstructions for MIPSI {}

/// The set of instructions defined in MIPS I.
/// 
/// The arguments must have been decoded prior to calling these.
pub trait MIPSIInstructions: MIPSICore {
    // Arithmetic

    /// Add signed
    fn add(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_add(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(Exception::ArithmeticOverflow);
        }
    }

    /// Add immediate signed
    fn addi(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16!(imm);
        if let Some(result) = source.checked_add(imm_32) {
            self.write_gp(tgt_reg, result);
        } else {
            self.trigger_exception(Exception::ArithmeticOverflow);
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
        let imm_32 = sign_extend_16!(imm);
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
            self.trigger_exception(Exception::ArithmeticOverflow);
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
        let source = sign_extend_32!(self.read_gp(src_reg));
        let target = sign_extend_32!(self.read_gp(tgt_reg));
        let result = source * target;
        self.write_hi(hi64!(result as u64));
        self.write_lo(lo64!(result as u64));
    }

    /// Multiply unsigned
    fn multu(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg) as u64;
        let target = self.read_gp(tgt_reg) as u64;
        let result = source * target;
        self.write_hi(hi64!(result));
        self.write_lo(lo64!(result));
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

    /// Set on less than
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

    /// Set on less than immediate
    fn slti(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg) as i32;
        let imm32 = sign_extend_16!(imm) as i32;
        let result = if source < imm32 {1} else {0};
        self.write_gp(tgt_reg, result);
    }

    /// Set on less than immediate unsigned
    fn sltiu(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm32 = sign_extend_16!(imm);
        let result = if source < imm32 {1} else {0};
        self.write_gp(tgt_reg, result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: make this a benchmark.
    #[test]
    fn add_speed() {
        use std::time::*;

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0);
        cpu.write_gp(2, 1);
        
        let start = SystemTime::now();
        
        for _ in 0..1_000_000 {
            cpu.add(1, 2, 1);
        }

        let time = start.elapsed().unwrap();

        println!("{} instructions per second.", 1_000_000.0 / time.as_secs_f64());

        assert_eq!(cpu.read_gp(1), 1_000_000);
    }
    
    #[test]
    fn add() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000);
        cpu.write_gp(2, 0x1234);
        cpu.add(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x2234);

        let mut cpu = MIPSI::default();

        // Test overflow.
        cpu.write_gp(1, 0xFFFFFFFF);
        cpu.write_gp(2, 0x5);
        cpu.add(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0);
    }

    #[test]
    fn addi() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000);
        cpu.addi(1, 2, 0x1001);
        assert_eq!(cpu.read_gp(2), 0x2001);

        let mut cpu = MIPSI::default();

        // Test overflow.
        cpu.write_gp(1, 0x10000);
        cpu.addi(1, 2, 0x8000);
        assert_eq!(cpu.read_gp(2), 0);
    }
    
    #[test]
    fn addu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000);
        cpu.write_gp(2, 0x1234);
        cpu.addu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x2234);

        let mut cpu = MIPSI::default();

        // Test overflow.
        cpu.write_gp(1, 0xFFFFFFFF);
        cpu.write_gp(2, 0x5);
        cpu.addu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 4);
    }

    #[test]
    fn addiu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000);
        cpu.addiu(1, 2, 0x1001);
        assert_eq!(cpu.read_gp(2), 0x2001);

        let mut cpu = MIPSI::default();

        // Test overflow.
        cpu.write_gp(1, 0x10000);
        cpu.addiu(1, 2, 0x8000);
        assert_eq!(cpu.read_gp(2), 0x8000);
    }
    
    #[test]
    fn sub() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x5555);
        cpu.write_gp(2, 0x1234);
        cpu.sub(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x4321);

        let mut cpu = MIPSI::default();

        // Test overflow.
        cpu.write_gp(1, 0xFFFFFFFE);
        cpu.write_gp(2, 0xFFFFFFFF);
        cpu.sub(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0);
    }
    
    #[test]
    fn subu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x5555);
        cpu.write_gp(2, 0x1234);
        cpu.subu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x4321);

        let mut cpu = MIPSI::default();

        // Test overflow.
        cpu.write_gp(1, 0xFFFFFFFE);
        cpu.write_gp(2, 0xFFFFFFFF);

        cpu.subu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), -1i32 as u32);
    }
    
    #[test]
    fn mult() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000_0000);
        cpu.write_gp(2, 0x2000_0000);
        cpu.mult(1, 2);
        assert_eq!(cpu.read_lo(), 0);
        assert_eq!(cpu.read_hi(), 0x200_0000);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.write_gp(2, 0x3);
        cpu.mult(1, 2);
        assert_eq!(cpu.read_lo(), 0xFFFF_FFFD);
        assert_eq!(cpu.read_hi(), 0xFFFF_FFFF);
    }

    #[test]
    fn multu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000_0000);
        cpu.write_gp(2, 0x2000_0000);
        cpu.multu(1, 2);
        assert_eq!(cpu.read_lo(), 0);
        assert_eq!(cpu.read_hi(), 0x200_0000);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.write_gp(2, 0x3);
        cpu.multu(1, 2);
        assert_eq!(cpu.read_lo(), 0xFFFF_FFFD);
        assert_eq!(cpu.read_hi(), 0x2);
    }

    #[test]
    fn div() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x8000_0000);
        cpu.write_gp(2, 0x2);
        cpu.div(1, 2);
        assert_eq!(cpu.read_lo(), 0xC000_0000);
        assert_eq!(cpu.read_hi(), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.write_gp(2, 0xFFFF_FFFE);
        cpu.div(1, 2);
        assert_eq!(cpu.read_lo(), 0);
        assert_eq!(cpu.read_hi(), 0xFFFF_FFFF);
    }

    #[test]
    fn divu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x8000_0000);
        cpu.write_gp(2, 0x2);
        cpu.divu(1, 2);
        assert_eq!(cpu.read_lo(), 0x4000_0000);
        assert_eq!(cpu.read_hi(), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.write_gp(2, 0xFFFF_FFFE);
        cpu.divu(1, 2);
        assert_eq!(cpu.read_lo(), 1);
        assert_eq!(cpu.read_hi(), 1);
    }

    #[test]
    fn and() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.write_gp(2, 0x0808_5555);
        cpu.and(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x0808_1111);
    }

    #[test]
    fn andi() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.andi(1, 2, 0xFFCC);
        assert_eq!(cpu.read_gp(2), 0x1100);
    }

    #[test]
    fn or() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.write_gp(2, 0x0808_5555);
        cpu.or(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x4F4F_5555);
    }

    #[test]
    fn ori() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.ori(1, 2, 0xFFCC);
        assert_eq!(cpu.read_gp(2), 0x4F4F_FFDD);
    }

    #[test]
    fn xor() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.write_gp(2, 0x0808_5555);
        cpu.xor(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0x4747_4444);
    }

    #[test]
    fn xori() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.xori(1, 2, 0xFFCC);
        assert_eq!(cpu.read_gp(2), 0x4F4F_EEDD);
    }

    #[test]
    fn nor() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x4F4F_1111);
        cpu.write_gp(2, 0x0808_5555);
        cpu.nor(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0xB0B0_AAAA);
    }

    #[test]
    fn sll() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x3);
        cpu.sll(1, 8, 2);
        assert_eq!(cpu.read_gp(2), 768);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234_5678);
        cpu.sll(1, 16, 2);
        assert_eq!(cpu.read_gp(2), 0x5678_0000);
    }

    #[test]
    fn srl() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFEDC_BA98);
        cpu.srl(1, 8, 2);
        assert_eq!(cpu.read_gp(2), 0xFE_DCBA);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234_5678);
        cpu.srl(1, 16, 2);
        assert_eq!(cpu.read_gp(2), 0x1234);
    }

    #[test]
    fn sra() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFEDC_BA98);
        cpu.sra(1, 8, 2);
        assert_eq!(cpu.read_gp(2), 0xFFFE_DCBA);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234_5678);
        cpu.sra(1, 16, 2);
        assert_eq!(cpu.read_gp(2), 0x1234);
    }

    #[test]
    fn sllv() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x3);
        cpu.write_gp(2, 0xFFFF_0001);
        cpu.sllv(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 6);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234_5678);
        cpu.write_gp(2, 0x3838_3838);
        cpu.sllv(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 0x7800_0000);
    }

    #[test]
    fn srlv() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x3);
        cpu.write_gp(2, 0xFFFF_0001);
        cpu.srlv(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 1);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234_5678);
        cpu.write_gp(2, 0x3838_3838);
        cpu.srlv(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 0x0000_0012);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x8765_4321);
        cpu.write_gp(2, 0x10);
        cpu.srlv(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 0x8765);
    }

    #[test]
    fn srav() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x3);
        cpu.write_gp(2, 0xFFFF_0001);
        cpu.srav(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 1);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234_5678);
        cpu.write_gp(2, 0x3838_3838);
        cpu.srav(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 0x0000_0012);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x8765_4321);
        cpu.write_gp(2, 0x10);
        cpu.srav(2, 1, 3);
        assert_eq!(cpu.read_gp(3), 0xFFFF_8765);
    }

    #[test]
    fn slt() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x100);
        cpu.write_gp(2, 0x100);
        cpu.slt(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.write_gp(2, 0x100);
        cpu.slt(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 1);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234);
        cpu.write_gp(2, 0x4321);
        cpu.slt(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 1);
    }

    #[test]
    fn sltu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x100);
        cpu.write_gp(2, 0x100);
        cpu.sltu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.write_gp(2, 0x100);
        cpu.sltu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1234);
        cpu.write_gp(2, 0x4321);
        cpu.sltu(1, 2, 3);
        assert_eq!(cpu.read_gp(3), 1);
    }

    #[test]
    fn slti() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000);
        cpu.slti(1, 2, 0x1000);
        assert_eq!(cpu.read_gp(2), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.slti(1, 2, 0x1);
        assert_eq!(cpu.read_gp(2), 1);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_0000);
        cpu.slti(1, 2, 0xFFFF);
        assert_eq!(cpu.read_gp(2), 1);
    }

    #[test]
    fn sltiu() {
        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0x1000);
        cpu.sltiu(1, 2, 0x1000);
        assert_eq!(cpu.read_gp(2), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_FFFF);
        cpu.sltiu(1, 2, 0x1);
        assert_eq!(cpu.read_gp(2), 0);

        let mut cpu = MIPSI::default();

        cpu.write_gp(1, 0xFFFF_0000);
        cpu.sltiu(1, 2, 0xFFFF);
        assert_eq!(cpu.read_gp(2), 1);
    }
}