use anyhow::Result;

use crate::asm::arch::Asm;
use crate::asm::exec::Arch;

pub struct Debug;

impl Arch for Debug {
    type IntReg = IntReg;
    type FloatReg = FloatReg;

    const INT_RET: Self::IntReg = IntReg { name: "int_acc" };
    const FLOAT_RET: Self::FloatReg = FloatReg { name: "int_acc" };

    const INT_ACC: Self::IntReg = IntReg { name: "int_acc" };
    const FLOAT_ACC: Self::FloatReg = FloatReg { name: "float_acc" };
    const INT_TMP: Self::IntReg = IntReg { name: "int_tmp" };
    const FLOAT_TMP: Self::FloatReg = FloatReg { name: "float_tmp" };

    fn movi(_: &mut Asm, from: Self::IntReg, to: Self::IntReg) {
        println!("movei {}, {}", to.name, from.name);
    }

    fn movf(_: &mut Asm, from: Self::FloatReg, to: Self::FloatReg) {
        println!("movef {}, {}", to.name, from.name);
    }

    fn storei(_: &mut Asm, reg: Self::IntReg, val: i64) {
        println!("movei {}, {}", reg.name, val);
    }

    fn storef(_: &mut Asm, reg: Self::FloatReg, val: f64) {
        println!("movef {}, {}", reg.name, val);
    }

    fn castf(_: &mut Asm, from: Self::IntReg, to: Self::FloatReg) {
        println!("movec {}, {}", to.name, from.name);
    }

    fn addi(_: &mut Asm, op: Self::IntReg) {
        println!("addi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn addf(_: &mut Asm, op: Self::FloatReg) {
        println!("addf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn subi(_: &mut Asm, op: Self::IntReg) {
        println!("subi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn subf(_: &mut Asm, op: Self::FloatReg) {
        println!("subf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn muli(_: &mut Asm, op: Self::IntReg) {
        println!("muli {}, {}", Self::INT_ACC.name, op.name);
    }

    fn mulf(_: &mut Asm, op: Self::FloatReg) {
        println!("mulf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn modi(_: &mut Asm, op: Self::IntReg) {
        println!("modi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn modf(_: &mut Asm, op: Self::FloatReg) {
        println!("modf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn divi(_: &mut Asm, op: Self::IntReg) {
        println!("divi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn divf(_: &mut Asm, op: Self::FloatReg) {
        println!("divf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn powi(_: &mut Asm, op: Self::IntReg) {
        println!("powi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn powf(_: &mut Asm, op: Self::FloatReg) {
        println!("powf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn ret(_: &mut Asm) {
        println!("ret");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntReg {
    name: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FloatReg {
    name: &'static str,
}
