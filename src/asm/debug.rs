use crate::asm::arch::Asm;
use crate::asm::exec::Arch;

pub struct Debug;

impl Arch for Debug {
    type Reg = DebugReg;
    const INT_RET: Self::Reg = DebugReg { name: "int_res" };
    const FLOAT_RET: Self::Reg = DebugReg { name: "float_res" };
    const INT_ACC: Self::Reg = DebugReg { name: "int_acc" };
    const FLOAT_ACC: Self::Reg = DebugReg { name: "float_acc" };
    const INT_TMP: Self::Reg = DebugReg { name: "int_tmp" };
    const FLOAT_TMP: Self::Reg = DebugReg { name: "float_tmp" };
    const INT_1: Self::Reg = DebugReg { name: "int_1" };
    const FLOAT_1: Self::Reg = DebugReg { name: "float_1" };
    const INT_2: Self::Reg = DebugReg { name: "int_2" };
    const FLOAT_2: Self::Reg = DebugReg { name: "float_2" };

    fn movei(asm: &mut Asm, from: Self::Reg, to: Self::Reg) {
        println!("movei {}, {}", to.name, from.name);
    }

    fn movef(asm: &mut Asm, from: Self::Reg, to: Self::Reg) {
        println!("movef {}, {}", to.name, from.name);
    }

    fn storei(_: &mut Asm, reg: Self::Reg, val: i128) {
        println!("movei {}, {}", reg.name, val)
    }

    fn storef(_: &mut Asm, reg: Self::Reg, val: f64) {
        println!("movef {}, {}", reg.name, val)
    }

    fn castf(_: &mut Asm, from: Self::Reg, to: Self::Reg) {
        println!("movec {}, {}", to.name, from.name)
    }

    fn addi(asm: &mut Asm, op: Self::Reg) {
        println!("addi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn addf(asm: &mut Asm, op: Self::Reg) {
        println!("addf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn subi(asm: &mut Asm, op: Self::Reg) {
        println!("subi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn subf(asm: &mut Asm, op: Self::Reg) {
        println!("subf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn muli(asm: &mut Asm, op: Self::Reg) {
        println!("muli {}, {}", Self::INT_ACC.name, op.name);
    }

    fn mulf(asm: &mut Asm, op: Self::Reg) {
        println!("mulf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn modi(asm: &mut Asm, op: Self::Reg) {
        println!("modi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn modf(asm: &mut Asm, op: Self::Reg) {
        println!("modf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn divi(asm: &mut Asm, op: Self::Reg) {
        println!("divi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn divf(asm: &mut Asm, op: Self::Reg) {
        println!("divf {}, {}", Self::FLOAT_ACC.name, op.name);
    }

    fn powi(asm: &mut Asm, op: Self::Reg) {
        println!("powi {}, {}", Self::INT_ACC.name, op.name);
    }

    fn powf(asm: &mut Asm, op: Self::Reg) {
        println!("powf {}, {}", Self::FLOAT_ACC.name, op.name);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DebugReg {
    name: &'static str,
}
