use crate::asm::arch::Asm;
use crate::parser::ast::{Exp, Op, Val};

pub trait Arch {
    type Reg: PartialEq + Eq;
    const INT_RET: Self::Reg;
    const FLOAT_RET: Self::Reg;

    const INT_ACC: Self::Reg;
    const FLOAT_ACC: Self::Reg;

    const INT_TMP: Self::Reg;
    const FLOAT_TMP: Self::Reg;

    const INT_1: Self::Reg;
    const FLOAT_1: Self::Reg;

    const INT_2: Self::Reg;
    const FLOAT_2: Self::Reg;

    fn movei(asm: &mut Asm, from: Self::Reg, to: Self::Reg);
    fn movef(asm: &mut Asm, from: Self::Reg, to: Self::Reg);

    fn storei(asm: &mut Asm, reg: Self::Reg, val: i128);
    fn storef(asm: &mut Asm, reg: Self::Reg, val: f64);

    fn castf(asm: &mut Asm, from: Self::Reg, to: Self::Reg);

    fn addi(asm: &mut Asm, op: Self::Reg);
    fn addf(asm: &mut Asm, op: Self::Reg);

    fn subi(asm: &mut Asm, op: Self::Reg);
    fn subf(asm: &mut Asm, op: Self::Reg);

    fn muli(asm: &mut Asm, op: Self::Reg);
    fn mulf(asm: &mut Asm, op: Self::Reg);

    fn modi(asm: &mut Asm, op: Self::Reg);
    fn modf(asm: &mut Asm, op: Self::Reg);

    fn divi(asm: &mut Asm, op: Self::Reg);
    fn divf(asm: &mut Asm, op: Self::Reg);

    fn powi(asm: &mut Asm, op: Self::Reg);
    fn powf(asm: &mut Asm, op: Self::Reg);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rt {
    Int,
    Float,
}

pub trait AsmCode {
    fn result_type(&self) -> Rt;
    fn to_asm<A: Arch>(&self, asm: &mut Asm, result: A::Reg);
}

impl AsmCode for Val {
    fn result_type(&self) -> Rt {
        match self {
            Val::Int(_) => Rt::Int,
            Val::Float(_) => Rt::Float,
        }
    }

    fn to_asm<A: Arch>(&self, asm: &mut Asm, result: <A as Arch>::Reg) {
        match self {
            Val::Int(val) => {
                A::storei(asm, result, *val);
            }
            Val::Float(val) => {
                A::storef(asm, result, *val);
            }
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

    fn to_asm<A: Arch>(&self, asm: &mut Asm, result: <A as Arch>::Reg) {
        match self {
            Exp::Val(val) => val.to_asm::<A>(asm, result),
            Exp::Exp { op, left, right } => {
                let left_rt = left.result_type();
                let right_rt = right.result_type();

                let need_cast = if left_rt != right_rt {
                    true
                } else {
                    false
                };

                let (l_reg, l_is_int) = if left_rt == Rt::Int {
                    left.to_asm::<A>(asm, A::INT_ACC);
                    if need_cast {
                        A::castf(asm, A::INT_ACC, A::FLOAT_ACC);
                        (A::FLOAT_ACC, false)
                    } else {
                        (A::INT_ACC, true)
                    }
                } else {
                    left.to_asm::<A>(asm, A::FLOAT_ACC);
                    (A::FLOAT_ACC, false)
                };

                let (r_reg, r_is_int) = if right_rt == Rt::Int {
                    right.to_asm::<A>(asm, A::INT_TMP);
                    if need_cast {
                        A::castf(asm, A::INT_TMP, A::FLOAT_TMP);
                        (A::FLOAT_TMP, false)
                    } else {
                        (A::INT_TMP, true)
                    }
                } else {
                    right.to_asm::<A>(asm, A::FLOAT_TMP);
                    (A::FLOAT_TMP, false)
                };

                match op {
                    Op::Add => {
                        if r_is_int {
                            A::addi(asm, A::INT_TMP);
                        } else {
                            A::addf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Sub => {
                        if r_is_int {
                            A::subi(asm, A::INT_TMP);
                        } else {
                            A::subf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Mul => {
                        if r_is_int {
                            A::muli(asm, A::INT_TMP);
                        } else {
                            A::mulf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Mod => {
                        if r_is_int {
                            A::modi(asm, A::INT_TMP);
                        } else {
                            A::modf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Div => {
                        if r_is_int {
                            A::divi(asm, A::INT_TMP);
                        } else {
                            A::divf(asm, A::FLOAT_TMP);
                        }
                    }
                    Op::Pow => {
                        if r_is_int {
                            A::powi(asm, A::INT_TMP);
                        } else {
                            A::powf(asm, A::FLOAT_TMP);
                        }
                    }
                }

                if l_reg != result {
                    if r_is_int {
                        A::movei(asm, l_reg, result);
                    } else {
                        A::movef(asm, l_reg, result);
                    }
                }
            }
        }
    }
}