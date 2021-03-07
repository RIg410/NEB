use anyhow::{anyhow, Error, Result};

use crate::asm::arch::Asm;
use crate::asm::exec::Arch;

pub struct X8664;

impl Arch for X8664 {
    type IntReg = IntReg;
    type FloatReg = FloatReg;

    const INT_RET: Self::IntReg = IntReg::RAX;
    const FLOAT_RET: Self::FloatReg = FloatReg::XMM0;

    const INT_ACC: Self::IntReg = IntReg::RAX;
    const FLOAT_ACC: Self::FloatReg = FloatReg::XMM0;

    const INT_TMP: Self::IntReg = IntReg::RBX;
    const FLOAT_TMP: Self::FloatReg = FloatReg::XMM1;

    fn movi(asm: &mut Asm, from: Self::IntReg, to: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn movf(asm: &mut Asm, from: Self::FloatReg, to: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn storei(asm: &mut Asm, reg: Self::IntReg, val: i64) {
        //mov
        asm.put(&[0x48]);
        // register
        match reg {
            IntReg::RAX => asm.put(&[0xb8]),
            IntReg::RBX => asm.put(&[0xb8]),
        }
        asm.put(&val.to_le_bytes());
    }

    fn storef(asm: &mut Asm, reg: Self::FloatReg, val: f64) {
        unimplemented!()
    }

    fn castf(asm: &mut Asm, from: Self::IntReg, to: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn addi(asm: &mut Asm, op: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn addf(asm: &mut Asm, op: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn subi(asm: &mut Asm, op: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn subf(asm: &mut Asm, op: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn muli(asm: &mut Asm, op: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn mulf(asm: &mut Asm, op: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn modi(asm: &mut Asm, op: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn modf(asm: &mut Asm, op: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn divi(asm: &mut Asm, op: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn divf(asm: &mut Asm, op: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn powi(asm: &mut Asm, op: Self::IntReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn powf(asm: &mut Asm, op: Self::FloatReg) -> Result<(), Error> {
        unimplemented!()
    }

    fn ret(asm: &mut Asm) {
        asm.put(&[0xc3]);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntReg {
    RAX,
    RBX,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FloatReg {
    XMM0,
    XMM1,
}
