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

    /// ADD
    fn add(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_add(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(Exception::ArithmeticOverflow);
        }
    }

    /// ADDI
    fn addi(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16!(imm);
        if let Some(result) = source.checked_add(imm_32) {
            self.write_gp(tgt_reg, result);
        } else {
            self.trigger_exception(Exception::ArithmeticOverflow);
        }
    }

    /// ADDU
    fn addu(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source.wrapping_add(target);
        self.write_gp(dst_reg, result);
    }

    /// ADDIU
    fn addiu(&mut self, src_reg: usize, tgt_reg: usize, imm: u16) {
        let source = self.read_gp(src_reg);
        let imm_32 = sign_extend_16!(imm);
        let result = source.wrapping_add(imm_32);
        self.write_gp(tgt_reg, result);
    }

    /// SUB
    fn sub(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        if let Some(result) = source.checked_sub(target) {
            self.write_gp(dst_reg, result);
        } else {
            self.trigger_exception(Exception::ArithmeticOverflow);
        }
    }

    /// SUBU
    fn subu(&mut self, src_reg: usize, tgt_reg: usize, dst_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        let result = source.wrapping_sub(target);
        self.write_gp(dst_reg, result);
    }

    /// MULT
    fn mult(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = sign_extend_32!(self.read_gp(src_reg));
        let target = sign_extend_32!(self.read_gp(tgt_reg));
        let result = source * target;
        self.write_hi(hi64!(result as u64));
        self.write_lo(lo64!(result as u64));
    }

    /// MULTU
    fn multu(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg) as u64;
        let target = self.read_gp(tgt_reg) as u64;
        let result = source * target;
        self.write_hi(hi64!(result));
        self.write_lo(lo64!(result));
    }

    /// DIV
    fn div(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg) as i32;
        let target = self.read_gp(tgt_reg) as i32;
        self.write_hi((source % target) as u32);
        self.write_lo((source / target) as u32);
    }

    /// DIVU
    fn divu(&mut self, src_reg: usize, tgt_reg: usize) {
        let source = self.read_gp(src_reg);
        let target = self.read_gp(tgt_reg);
        self.write_hi(source % target);
        self.write_lo(source / target);
    }

    /// MFHI
    fn mfhi(&mut self, dst_reg: usize) {
        self.write_gp(dst_reg, self.read_hi());
    }

    /// MTHI
    fn mthi(&mut self, src_reg: usize) {
        self.write_hi(self.read_gp(src_reg));
    }

    /// MFLO
    fn mflo(&mut self, dst_reg: usize) {
        self.write_gp(dst_reg, self.read_lo());
    }

    /// MTLO
    fn mtlo(&mut self, src_reg: usize) {
        self.write_lo(self.read_gp(src_reg));
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
}