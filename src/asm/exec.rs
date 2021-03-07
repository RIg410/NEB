use anyhow::Result;

use crate::asm::arch::Asm;
use crate::parser::ast::{Exp, Op, Val};

pub trait Arch {
    type IntReg: PartialEq + Eq;
    type FloatReg: PartialEq + Eq;

    const INT_RET: Self::IntReg;
    const FLOAT_RET: Self::FloatReg;

    const INT_ACC: Self::IntReg;
    const FLOAT_ACC: Self::FloatReg;

    const INT_TMP: Self::IntReg;
    const FLOAT_TMP: Self::FloatReg;

    fn movi(asm: &mut Asm, from: Self::IntReg, to: Self::IntReg);
    fn movf(asm: &mut Asm, from: Self::FloatReg, to: Self::FloatReg);

    fn storei(asm: &mut Asm, reg: Self::IntReg, val: i64);
    fn storef(asm: &mut Asm, reg: Self::FloatReg, val: f64);

    fn castf(asm: &mut Asm, from: Self::IntReg, to: Self::FloatReg);

    fn addi(asm: &mut Asm, op: Self::IntReg);
    fn addf(asm: &mut Asm, op: Self::FloatReg);

    fn subi(asm: &mut Asm, op: Self::IntReg);
    fn subf(asm: &mut Asm, op: Self::FloatReg);

    fn muli(asm: &mut Asm, op: Self::IntReg);
    fn mulf(asm: &mut Asm, op: Self::FloatReg);

    fn modi(asm: &mut Asm, op: Self::IntReg);
    fn modf(asm: &mut Asm, op: Self::FloatReg);

    fn divi(asm: &mut Asm, op: Self::IntReg);
    fn divf(asm: &mut Asm, op: Self::FloatReg);

    fn powi(asm: &mut Asm, op: Self::IntReg);
    fn powf(asm: &mut Asm, op: Self::FloatReg);

    fn ret(asm: &mut Asm);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rt {
    Int,
    Float,
}

pub trait AsmCode {
    fn result_type(&self) -> Rt;
    fn to_asm<A: Arch>(&self, asm: &mut Asm, int: A::IntReg, float: A::FloatReg);
}

impl AsmCode for Val {
    fn result_type(&self) -> Rt {
        match self {
            Val::Int(_) => Rt::Int,
            Val::Float(_) => Rt::Float,
        }
    }

    fn to_asm<A: Arch>(&self, asm: &mut Asm, int: A::IntReg, float: A::FloatReg) {
        match self {
            Val::Int(val) => A::storei(asm, int, *val),
            Val::Float(val) => A::storef(asm, float, *val),
        }
    }
}

impl AsmCode for Exp {
    fn result_type(&self) -> Rt {
        match self {
            Exp::Val(val) => val.result_type(),
            Exp::Exp { op: _, left, right } => {
                if left.result_type() == Rt::Float || right.result_type() == Rt::Float {
                    Rt::Float
                } else {
                    Rt::Int
                }
            }
        }
    }

    fn to_asm<A: Arch>(&self, asm: &mut Asm, int: A::IntReg, float: A::FloatReg) {
        match self {
            Exp::Val(val) => val.to_asm::<A>(asm, int, float),
            Exp::Exp { op, left, right } => {
                let left_rt = left.result_type();
                let right_rt = right.result_type();

                let int_result = left_rt == Rt::Int && right_rt == Rt::Int;

                let need_cast = if left_rt != right_rt { true } else { false };

                if left_rt == Rt::Int {
                    left.to_asm::<A>(asm, A::INT_ACC, A::FLOAT_ACC);
                    if need_cast {
                        A::castf(asm, A::INT_ACC, A::FLOAT_ACC);
                    }
                } else {
                    left.to_asm::<A>(asm, A::INT_ACC, A::FLOAT_ACC);
                }

                if right_rt == Rt::Int {
                    right.to_asm::<A>(asm, A::INT_TMP, A::FLOAT_TMP);
                    if need_cast {
                        A::castf(asm, A::INT_TMP, A::FLOAT_TMP);
                    }
                } else {
                    right.to_asm::<A>(asm, A::INT_TMP, A::FLOAT_TMP);
                }

                match op {
                    Op::Add => {
                        if int_result {
                            A::addi(asm, A::INT_TMP);
                        } else {
                            A::addf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Sub => {
                        if int_result {
                            A::subi(asm, A::INT_TMP);
                        } else {
                            A::subf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Mul => {
                        if int_result {
                            A::muli(asm, A::INT_TMP);
                        } else {
                            A::mulf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Mod => {
                        if int_result {
                            A::modi(asm, A::INT_TMP);
                        } else {
                            A::modf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Div => {
                        if int_result {
                            A::divi(asm, A::INT_TMP);
                        } else {
                            A::divf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Pow => {
                        if int_result {
                            A::powi(asm, A::INT_TMP);
                        } else {
                            A::powf(asm, A::FLOAT_TMP);
                        }
                    }
                }

                if int_result {
                    if A::INT_ACC != int {
                        A::movi(asm, A::INT_ACC, int);
                    }
                } else {
                    if A::FLOAT_ACC != float {
                        A::movf(asm, A::FLOAT_ACC, float);
                    }
                }
            }
        }
    }
}
