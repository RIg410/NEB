use crate::asm::arch::Asm;
use crate::asm::exec::Arch;

pub struct X8664;


impl Arch for X8664 {
    type Reg = X86reg;
    const INT_RET: Self::Reg = X86reg {};
    const FLOAT_RET: Self::Reg = X86reg {};
    const INT_ACC: Self::Reg = X86reg {};
    const FLOAT_ACC: Self::Reg = X86reg {};
    const INT_TMP: Self::Reg = X86reg {};
    const FLOAT_TMP: Self::Reg = X86reg {};
    const INT_1: Self::Reg = X86reg {};
    const FLOAT_1: Self::Reg = X86reg {};
    const INT_2: Self::Reg = X86reg {};
    const FLOAT_2: Self::Reg = X86reg {};

    fn movei(asm: &mut Asm, from: Self::Reg, to: Self::Reg) {
        unimplemented!()
    }

    fn movef(asm: &mut Asm, from: Self::Reg, to: Self::Reg) {
        unimplemented!()
    }

    fn storei(_asm: &mut Asm, _reg: Self::Reg, _val: i128) {
        unimplemented!()
    }

    fn storef(_asm: &mut Asm, _reg: Self::Reg, _val: f64) {
        unimplemented!()
    }

    fn castf(_asm: &mut Asm, _from: Self::Reg, _to: Self::Reg) {
        unimplemented!()
    }

    fn addi(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn addf(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn subi(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn subf(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn muli(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn mulf(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn modi(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn modf(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn divi(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn divf(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn powi(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }

    fn powf(asm: &mut Asm, oper: Self::Reg) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct X86reg {}
