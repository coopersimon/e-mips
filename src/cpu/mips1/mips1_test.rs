use crate::{
    coproc::*,
    mem::*,
    cpu::mips1::*,
    cpu::MIPSICore,
    cpu::MIPSCore
};

struct LittleMemTest {
    bytes: Vec<u8>
}

impl LittleMemTest {
    fn new(size: usize) -> Self {
        Self {
            bytes: vec![0; size]
        }
    }
}

impl Mem32 for LittleMemTest {
    type Addr = u32;
    const LITTLE_ENDIAN: bool = true;

    fn clock(&mut self, _cycles: usize) -> u8 {
        0
    }

    fn read_byte(&mut self, addr: Self::Addr) -> Data<u8> {
        Data {
            data: self.bytes[addr as usize],
            cycles: 1
        }
    }

    fn write_byte(&mut self, addr: Self::Addr, data: u8) -> usize {
        self.bytes[addr as usize] = data;
        1
    }
}

#[derive(Default)]
struct TestCoproc0 {
    data_reg:       [u32; 32],
    last_exception: Option<ExceptionCode>,
}

impl Coprocessor0 for TestCoproc0 {
    fn move_from_reg(&mut self, reg: u8) -> u32 {
        self.data_reg[reg as usize]
    }
    fn move_to_reg(&mut self, reg: u8, val: u32) {
        self.data_reg[reg as usize] = val;
    }

    fn operation(&mut self, op: u32) {
        match op {
            _ => {}
        }
    }

    fn external_interrupt(&mut self, _mask: u8) -> bool {
        false
    }

    fn reset(&mut self) -> u32 {
        0
    }

    fn trigger_exception(&mut self, exception: &Exception) -> u32 {
        self.last_exception = Some(exception.code);
        0
    }
}

#[derive(Default)]
struct TestCoproc {
    control_reg:    [u32; 32],
    data_reg:       [u32; 32],
}

impl Coprocessor for TestCoproc {
    fn move_from_reg(&mut self, reg: u8) -> u32 {
        self.data_reg[reg as usize]
    }
    fn move_to_reg(&mut self, reg: u8, val: u32) {
        self.data_reg[reg as usize] = val;
    }

    fn move_from_control(&mut self, reg: u8) -> u32 {
        self.control_reg[reg as usize]
    }
    fn move_to_control(&mut self, reg: u8, val: u32) {
        self.control_reg[reg as usize] = val;
    }

    fn load_from_mem(&mut self, reg: u8, val: u32) {
        self.move_to_reg(reg, val);
    }
    fn store_to_mem(&mut self, reg: u8) -> u32 {
        self.move_from_reg(reg)
    }

    // For testing purposes:
    // op "1" adds data reg 1 & 2 together, and stores result in 3
    // op "2" multiplies data reg 4 & 5 together, and stores result in 6
    fn operation(&mut self, op: u32) {
        match op {
            1 => self.data_reg[3] = self.data_reg[1] + self.data_reg[2],
            2 => self.data_reg[6] = self.data_reg[4] * self.data_reg[5],
            _ => {}
        }
    }
}

impl MIPSI<LittleMemTest, TestCoproc0, TestCoproc, EmptyCoproc, EmptyCoproc> {
    fn default() -> Self {
        Self::with_memory(Box::new(LittleMemTest::new(0x1000)))
            .add_coproc0(TestCoproc0::default())
            .add_coproc1(TestCoproc::default())
            .build()
    }
}

fn make_i_instr(instr: u32, src: u32, tgt: u32, imm: u32) -> u32 {
    (instr << 26) | (src << 21) | (tgt << 16) | imm
}

// TODO: make this a benchmark.
#[test]
fn add_speed() {
    use std::time::*;

    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0);
    cpu.write_gp(2, 1);
    
    let start = SystemTime::now();
    
    for _ in 0..1_000_000 {
        cpu.div(1, 2);
    }

    let time = start.elapsed().unwrap();

    println!("{} instructions per second.", 1_000_000.0 / time.as_secs_f64());

    //assert_eq!(cpu.read_gp(1), 1_000_000);
}

#[test]
fn add() {
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x1000);
    cpu.write_gp(2, 0x1234);
    cpu.add(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0x2234);

    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0xFFFFFFFF);
    cpu.write_gp(2, 0x5);
    cpu.add(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 4);
    assert!(cpu.coproc0.last_exception.is_none());

    // Test overflow.
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x8000_0000);
    cpu.write_gp(2, 0xFFFF_0000);
    cpu.add(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0);
    assert_eq!(cpu.coproc0.last_exception, Some(ExceptionCode::ArithmeticOverflow));
}

#[test]
fn addi() {
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x1000);
    cpu.addi(1, 2, 0x1001);
    assert_eq!(cpu.read_gp(2), 0x2001);

    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x10000);
    cpu.addi(1, 2, 0x8000);
    assert_eq!(cpu.read_gp(2), 0x8000);
    assert!(cpu.coproc0.last_exception.is_none());

    // Test overflow.
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x7FFF_FFFF);
    cpu.addi(1, 2, 0x100);
    assert_eq!(cpu.read_gp(2), 0);
    assert_eq!(cpu.coproc0.last_exception, Some(ExceptionCode::ArithmeticOverflow));
}

#[test]
fn addu() {
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x1000);
    cpu.write_gp(2, 0x1234);
    cpu.addu(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0x2234);

    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0xFFFFFFFF);
    cpu.write_gp(2, 0x5);
    cpu.addu(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 4);
    assert!(cpu.coproc0.last_exception.is_none());

    // Test overflow.
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x8000_0000);
    cpu.write_gp(2, 0xFFFF_0000);
    cpu.addu(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0x7FFF_0000);
    assert_eq!(cpu.coproc0.last_exception, None);
}

#[test]
fn addiu() {
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x1000);
    cpu.addiu(1, 2, 0x1001);
    assert_eq!(cpu.read_gp(2), 0x2001);

    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x10000);
    cpu.addiu(1, 2, 0x8000);
    assert_eq!(cpu.read_gp(2), 0x8000);
    assert!(cpu.coproc0.last_exception.is_none());

    // Test overflow.
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x7FFF_FFFF);
    cpu.addiu(1, 2, 0x100);
    assert_eq!(cpu.read_gp(2), 0x8000_00FF);
    assert_eq!(cpu.coproc0.last_exception, None);
}

#[test]
fn sub() {
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x5555);
    cpu.write_gp(2, 0x1234);
    cpu.sub(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0x4321);

    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0xFFFFFFFE);
    cpu.write_gp(2, 0xFFFFFFFF);
    cpu.sub(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0xFFFF_FFFF);
    assert!(cpu.coproc0.last_exception.is_none());

    // Test overflow.
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x8000_0000);
    cpu.write_gp(2, 0x1);
    cpu.sub(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0);
    assert_eq!(cpu.coproc0.last_exception, Some(ExceptionCode::ArithmeticOverflow));
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
    assert_eq!(cpu.read_gp(3), 0xFFFF_FFFF);
    assert!(cpu.coproc0.last_exception.is_none());

    // Test overflow.
    let mut cpu = MIPSI::default();

    cpu.write_gp(1, 0x8000_0000);
    cpu.write_gp(2, 0x1);
    cpu.subu(1, 2, 3);
    assert_eq!(cpu.read_gp(3), 0x7FFF_FFFF);
    assert_eq!(cpu.coproc0.last_exception, None);
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

#[test]
fn lb() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);

    cpu.write_gp(1, 0);
    cpu.lb(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x21);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);

    cpu.write_gp(1, 8);
    cpu.lb(1, 2, 3);
    assert_eq!(cpu.read_gp(2), 0xFFFF_FFFE);
}

#[test]
fn lbu() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);

    cpu.write_gp(1, 2);
    cpu.lbu(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x65);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);

    cpu.write_gp(1, 8);
    cpu.lbu(1, 2, 3);
    assert_eq!(cpu.read_gp(2), 0xFE);
}

#[test]
fn lh() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);

    cpu.write_gp(1, 0);
    cpu.lh(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x4321);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);

    cpu.write_gp(1, 8);
    cpu.lh(1, 2, 2);
    assert_eq!(cpu.read_gp(2), 0xFFFF_FEDC);
}

#[test]
fn lhu() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);

    cpu.write_gp(1, 2);
    cpu.lhu(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x8765);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);

    cpu.write_gp(1, 8);
    cpu.lhu(1, 2, 2);
    assert_eq!(cpu.read_gp(2), 0xFEDC);
}

#[test]
fn lw() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);

    cpu.write_gp(1, 0);
    cpu.lw(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x8765_4321);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);

    cpu.write_gp(1, 6);
    cpu.lw(1, 2, 2);
    assert_eq!(cpu.read_gp(2), 0xFEDC_BA98);
}

#[test]
fn lwl() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    assert_eq!(cpu.mut_mem().read_byte(0).data, 0x21);

    cpu.write_gp(1, 1);
    cpu.lwl(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x4321_0000);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0xFEDC_BA98);
    assert_eq!(cpu.mut_mem().read_byte(3).data, 0xFE);

    cpu.write_gp(1, 1);
    cpu.lwl(1, 2, 1);
    assert_eq!(cpu.read_gp(2), 0xDCBA_9800);
}

#[test]
fn lwr() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    assert_eq!(cpu.mut_mem().read_byte(0).data, 0x21);

    cpu.write_gp(1, 1);
    cpu.lwr(1, 2, 0);
    assert_eq!(cpu.read_gp(2), 0x87_6543);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0xFEDC_BA98);
    assert_eq!(cpu.mut_mem().read_byte(3).data, 0xFE);

    cpu.write_gp(1, 1);
    cpu.lwr(1, 2, 1);
    assert_eq!(cpu.read_gp(2), 0x0000_FEDC);
}

#[test]
fn sb() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    cpu.write_gp(2, 0xABCD);

    cpu.write_gp(1, 0);
    cpu.sb(1, 2, 0);
    assert_eq!(cpu.mut_mem().read_word(0).data, 0x8765_43CD);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);
    cpu.write_gp(2, 0x1234_5678);

    cpu.write_gp(1, 8);
    cpu.sb(1, 2, 3);
    assert_eq!(cpu.mut_mem().read_word(8).data, 0x78DC_BA98);
}

#[test]
fn sh() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    cpu.write_gp(2, 0xABCD);

    cpu.write_gp(1, 0);
    cpu.sh(1, 2, 0);
    assert_eq!(cpu.mut_mem().read_word(0).data, 0x8765_ABCD);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);
    cpu.write_gp(2, 0x1234_5678);

    cpu.write_gp(1, 8);
    cpu.sh(1, 2, 2);
    assert_eq!(cpu.mut_mem().read_word(8).data, 0x5678_BA98);
}

#[test]
fn sw() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    cpu.write_gp(2, 0xABCD);

    cpu.write_gp(1, 0);
    cpu.sw(1, 2, 0);
    assert_eq!(cpu.mut_mem().read_word(0).data, 0xABCD);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);
    cpu.write_gp(2, 0x1234_5678);

    cpu.write_gp(1, 6);
    cpu.sw(1, 2, 2);
    assert_eq!(cpu.mut_mem().read_word(8).data, 0x1234_5678);
}

#[test]
fn swl() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    cpu.write_gp(2, 0xABCD);

    cpu.write_gp(1, 1);
    cpu.swl(1, 2, 0);
    assert_eq!(cpu.mut_mem().read_word(0).data, 0x8765_0000);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);
    cpu.write_gp(2, 0x1234_5678);

    cpu.write_gp(1, 8);
    cpu.swl(1, 2, 2);
    assert_eq!(cpu.mut_mem().read_word(8).data, 0xFE12_3456);
}

#[test]
fn swr() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, 0x8765_4321);
    cpu.write_gp(2, 0xABCD);

    cpu.write_gp(1, 1);
    cpu.swr(1, 2, 0);
    assert_eq!(cpu.mut_mem().read_word(0).data, 0x00AB_CD21);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(8, 0xFEDC_BA98);
    cpu.write_gp(2, 0x1234_5678);

    cpu.write_gp(1, 8);
    cpu.swr(1, 2, 2);
    assert_eq!(cpu.mut_mem().read_word(8).data, 0x5678_BA98);
}

// These tests test the operation of branches, but also rely on the correct behaviour of:
// - ADDI
// - Step

#[test]
fn beq() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, make_i_instr(0x04, 1, 2, 0x40));
    cpu.mut_mem().write_word(4, make_i_instr(0x08, 3, 3, 0x123));
    cpu.mut_mem().write_word(8, make_i_instr(0x08, 4, 4, 0x123));
    cpu.mut_mem().write_word(0x104, make_i_instr(0x8, 4, 4, 0x456));
    cpu.write_gp(1, 0x1234);
    cpu.write_gp(2, 0x1234);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.read_gp(3), 0x123);
    cpu.step();
    assert_eq!(cpu.read_gp(4), 0x456);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, make_i_instr(0x04, 1, 2, 0x40));
    cpu.mut_mem().write_word(4, make_i_instr(0x08, 3, 3, 0x123));
    cpu.mut_mem().write_word(8, make_i_instr(0x08, 4, 4, 0x123));
    cpu.mut_mem().write_word(0x104, make_i_instr(0x8, 4, 4, 0x456));
    cpu.write_gp(1, 0x1234);
    cpu.write_gp(2, 0);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.read_gp(3), 0x123);
    cpu.step();
    assert_eq!(cpu.read_gp(4), 0x123);
}

#[test]
fn bgtz() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, make_i_instr(0x07, 1, 0, 0x40));
    cpu.mut_mem().write_word(4, make_i_instr(0x08, 3, 3, 0x123));
    cpu.mut_mem().write_word(8, make_i_instr(0x08, 4, 4, 0x123));
    cpu.mut_mem().write_word(0x104, make_i_instr(0x8, 4, 4, 0x456));
    cpu.write_gp(1, 0x1234_5678);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.read_gp(3), 0x123);
    cpu.step();
    assert_eq!(cpu.read_gp(4), 0x456);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, make_i_instr(0x07, 1, 2, 0x40));
    cpu.mut_mem().write_word(4, make_i_instr(0x08, 3, 3, 0x123));
    cpu.mut_mem().write_word(8, make_i_instr(0x08, 4, 4, 0x123));
    cpu.mut_mem().write_word(0x104, make_i_instr(0x8, 4, 4, 0x456));
    cpu.write_gp(1, 0);

    cpu.step();
    cpu.step();
    assert_eq!(cpu.read_gp(3), 0x123);
    cpu.step();
    assert_eq!(cpu.read_gp(4), 0x123);
}

#[test]
fn bgezal() {
    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, make_i_instr(0x01, 1, 0x11, 0x40));
    cpu.mut_mem().write_word(4, make_i_instr(0x08, 3, 3, 0x123));
    cpu.mut_mem().write_word(8, make_i_instr(0x08, 4, 4, 0x123));
    cpu.mut_mem().write_word(0x104, make_i_instr(0x8, 4, 4, 0x456));
    cpu.write_gp(1, 0x1234_5678);

    cpu.step();
    assert_eq!(cpu.read_gp(31), 8);
    cpu.step();
    assert_eq!(cpu.read_gp(3), 0x123);
    cpu.step();
    assert_eq!(cpu.read_gp(4), 0x456);

    let mut cpu = MIPSI::default();

    cpu.mut_mem().write_word(0, make_i_instr(0x01, 1, 0x11, 0x40));
    cpu.mut_mem().write_word(4, make_i_instr(0x08, 3, 3, 0x123));
    cpu.mut_mem().write_word(8, make_i_instr(0x08, 4, 4, 0x123));
    cpu.mut_mem().write_word(0x104, make_i_instr(0x8, 4, 4, 0x456));
    cpu.write_gp(1, 0xFFFF_FFFF);

    cpu.step();
    assert_eq!(cpu.read_gp(31), 8);
    cpu.step();
    assert_eq!(cpu.read_gp(3), 0x123);
    cpu.step();
    assert_eq!(cpu.read_gp(4), 0x123);
}

// TODO: test jumps

#[test]
fn mtc1() {
    let mut cpu = MIPSI::default();

    cpu.write_gp(10, 0x100);
    cpu.mtcz(Coproc::_1, 10, 1);
    
    assert_eq!(cpu.coproc_1().unwrap().data_reg[1], 0x100);
}

#[test]
fn mfc1() {
    let mut cpu = MIPSI::default();

    cpu.coproc_1().unwrap().data_reg[1] = 0xFF;
    cpu.mfcz(Coproc::_1, 10, 1);
    
    assert_eq!(cpu.read_gp(10), 0xFF);
}

#[test]
fn cop1() {
    let mut cpu = MIPSI::default();

    cpu.coproc_1().unwrap().data_reg[1] = 0xFF;
    cpu.coproc_1().unwrap().data_reg[2] = 0x2;

    cpu.mut_mem().write_word(0, (0x11 << 26) | (0x1 << 25) | 0x1);

    cpu.step();
    
    assert_eq!(cpu.coproc_1().unwrap().data_reg[3], 0x101);
}
