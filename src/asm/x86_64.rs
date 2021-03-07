use anyhow::{anyhow, Error, Result};

use crate::asm::arch::Asm;
use crate::asm::exec::Arch;

pub struct X8664 {
    asm: Asm,
}

impl Arch for X8664 {
    type IntReg = IntReg;
    type FloatReg = FloatReg;

    const INT_RET: Self::IntReg = IntReg::RAX;
    const FLOAT_RET: Self::FloatReg = FloatReg::XMM0;

    const INT_ACC: Self::IntReg = IntReg::RAX;
    const FLOAT_ACC: Self::FloatReg = FloatReg::XMM0;

    const INT_TMP: Self::IntReg = IntReg::RCX;
    const FLOAT_TMP: Self::FloatReg = FloatReg::XMM1;

    fn movi(asm: &mut Asm, from: Self::IntReg, to: Self::IntReg) {
        if from == to {
            return;
        }
        asm.put(&[0x48, 0x89]);

        if from == IntReg::RAX && to == IntReg::RCX {
            asm.put(&[0xc1]);
            return;
        }

        if from == IntReg::RCX && to == IntReg::RAX {
            asm.put(&[0xc8]);
        }
    }

    fn movf(asm: &mut Asm, from: Self::FloatReg, to: Self::FloatReg)  {
        unimplemented!()
    }

    fn storei(asm: &mut Asm, reg: Self::IntReg, val: i64) {
        asm.put(&[0x48]);
        match reg {
            IntReg::RAX => asm.put(&[0xb8]),
            IntReg::RCX => asm.put(&[0xb9]),
        }
        asm.put(&val.to_le_bytes());
    }

    fn storef(asm: &mut Asm, reg: Self::FloatReg, val: f64) {
        unimplemented!()
    }

    fn castf(asm: &mut Asm, from: Self::IntReg, to: Self::FloatReg) {
        unimplemented!()
    }

    fn addi(asm: &mut Asm, op: Self::IntReg) {
        asm.put(&[0x48, 0x01]);
        match op {
            IntReg::RAX => asm.put(&[0xc0]),
            IntReg::RCX => asm.put(&[0xc8]),
        }
    }

    fn addf(asm: &mut Asm, op: Self::FloatReg)  {
        unimplemented!()
    }

    fn subi(asm: &mut Asm, op: Self::IntReg)  {
        unimplemented!()
    }

    fn subf(asm: &mut Asm, op: Self::FloatReg)  {
        unimplemented!()
    }

    fn muli(asm: &mut Asm, op: Self::IntReg) {
        asm.put(&[0x48, 0xf7]);
        match op {
            IntReg::RAX => asm.put(&[0xe8]),
            IntReg::RCX => asm.put(&[0xe9]),
        }
    }

    fn mulf(asm: &mut Asm, op: Self::FloatReg)  {
        unimplemented!()
    }

    fn modi(asm: &mut Asm, op: Self::IntReg)  {
        unimplemented!()
    }

    fn modf(asm: &mut Asm, op: Self::FloatReg) {
        unimplemented!()
    }

    fn divi(asm: &mut Asm, op: Self::IntReg) {
        asm.put(&[0x48, 0xf7]);
        match op {
            IntReg::RAX => asm.put(&[0xf8]),
            IntReg::RCX => asm.put(&[0xf9]),
        }
    }

    fn divf(asm: &mut Asm, op: Self::FloatReg) {
        unimplemented!()
    }

    fn powi(asm: &mut Asm, op: Self::IntReg)  {
        unimplemented!()
    }

    fn powf(asm: &mut Asm, op: Self::FloatReg)  {
        unimplemented!()
    }

    fn popi(asm: &mut Asm, reg: Self::IntReg) {
        unimplemented!()
    }

    fn popf(asm: &mut Asm, reg: Self::FloatReg) {
        unimplemented!()
    }

    fn pushli(asm: &mut Asm, val: i64) {
        unimplemented!()
    }

    fn pushlf(asm: &mut Asm, val: f64) {
        unimplemented!()
    }

    fn pushi(asm: &mut Asm, reg: Self::IntReg) {
        unimplemented!()
    }

    fn pushf(asm: &mut Asm, reg: Self::FloatReg) {
        unimplemented!()
    }

    fn ret(asm: &mut Asm) {
        asm.put(&[0xc3]);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntReg {
    RAX,
    RCX,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FloatReg {
    XMM0,
    XMM1,
}
