use crate::asm::arch::Asm;
use crate::asm::exec::Arch;
use anyhow::Result;

pub struct Debug;

impl Arch for Debug {
    type IntReg = IntReg;
    type FloatReg = FloatReg;

    const INT_RET: Self::IntReg = IntReg { name: "int_res" };
    const FLOAT_RET: Self::FloatReg = FloatReg { name: "float_res" };
    const INT_ACC: Self::IntReg = IntReg { name: "int_acc" };
    const FLOAT_ACC: Self::FloatReg = FloatReg { name: "float_acc" };
    const INT_TMP: Self::IntReg = IntReg { name: "int_tmp" };
    const FLOAT_TMP: Self::FloatReg = FloatReg { name: "float_tmp" };

    fn movi(_: &mut Asm, from: Self::IntReg, to: Self::IntReg) -> Result<()> {
        println!("movei {}, {}", to.name, from.name);
        Ok(())
    }

    fn movf(_: &mut Asm, from: Self::FloatReg, to: Self::FloatReg) -> Result<()> {
        println!("movef {}, {}", to.name, from.name);
        Ok(())
    }

    fn storei(_: &mut Asm, reg: Self::IntReg, val: i64) {
        println!("movei {}, {}", reg.name, val);
    }

    fn storef(_: &mut Asm, reg: Self::FloatReg, val: f64) {
        println!("movef {}, {}", reg.name, val);
    }

    fn castf(_: &mut Asm, from: Self::IntReg, to: Self::FloatReg) -> Result<()> {
        println!("movec {}, {}", to.name, from.name);
        Ok(())
    }

    fn addi(_: &mut Asm, op: Self::IntReg) -> Result<()> {
        println!("addi {}, {}", Self::INT_ACC.name, op.name);
        Ok(())
    }

    fn addf(_: &mut Asm, op: Self::FloatReg) -> Result<()> {
        println!("addf {}, {}", Self::FLOAT_ACC.name, op.name);
        Ok(())
    }

    fn subi(_: &mut Asm, op: Self::IntReg) -> Result<()> {
        println!("subi {}, {}", Self::INT_ACC.name, op.name);
        Ok(())
    }

    fn subf(_: &mut Asm, op: Self::FloatReg) -> Result<()> {
        println!("subf {}, {}", Self::FLOAT_ACC.name, op.name);
        Ok(())
    }

    fn muli(_: &mut Asm, op: Self::IntReg) -> Result<()> {
        println!("muli {}, {}", Self::INT_ACC.name, op.name);
        Ok(())
    }

    fn mulf(_: &mut Asm, op: Self::FloatReg) -> Result<()> {
        println!("mulf {}, {}", Self::FLOAT_ACC.name, op.name);
        Ok(())
    }

    fn modi(_: &mut Asm, op: Self::IntReg) -> Result<()> {
        println!("modi {}, {}", Self::INT_ACC.name, op.name);
        Ok(())
    }

    fn modf(_: &mut Asm, op: Self::FloatReg) -> Result<()> {
        println!("modf {}, {}", Self::FLOAT_ACC.name, op.name);
        Ok(())
    }

    fn divi(_: &mut Asm, op: Self::IntReg) -> Result<()> {
        println!("divi {}, {}", Self::INT_ACC.name, op.name);
        Ok(())
    }

    fn divf(_: &mut Asm, op: Self::FloatReg) -> Result<()> {
        println!("divf {}, {}", Self::FLOAT_ACC.name, op.name);
        Ok(())
    }

    fn powi(_: &mut Asm, op: Self::IntReg) -> Result<()> {
        println!("powi {}, {}", Self::INT_ACC.name, op.name);
        Ok(())
    }

    fn powf(_: &mut Asm, op: Self::FloatReg) -> Result<()> {
        println!("powf {}, {}", Self::FLOAT_ACC.name, op.name);
        Ok(())
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
