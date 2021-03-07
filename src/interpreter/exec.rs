use crate::interpreter::Execution;
use crate::parser::ast::{Exp, Op, Val};

impl Execution for Val {
    fn exec(&self) -> Val {
        *self
    }
}

impl Execution for Exp {
    fn exec(&self) -> Val {
        match self {
            Exp::Val(val) => *val,
            Exp::Exp { op, left, right } => {
                let (left, right) = unify_types(left.exec(), right.exec());
                match op {
                    Op::Add => {
                        match (left, right) {
                            (Val::Float(l), Val::Float(r)) => { Val::Float(l + r) }
                            (Val::Int(l), Val::Int(r)) => { Val::Int(l + r) }
                            _ => panic!("invalid invariant")
                        }
                    }
                    Op::Sub => {
                        match (left, right) {
                            (Val::Float(l), Val::Float(r)) => { Val::Float(l - r) }
                            (Val::Int(l), Val::Int(r)) => { Val::Int(l - r) }
                            _ => panic!("invalid invariant")
                        }
                    }
                    Op::Mul => {
                        match (left, right) {
                            (Val::Float(l), Val::Float(r)) => { Val::Float(l * r) }
                            (Val::Int(l), Val::Int(r)) => { Val::Int(l * r) }
                            _ => panic!("invalid invariant")
                        }
                    }
                    Op::Mod => {
                        match (left, right) {
                            (Val::Float(l), Val::Float(r)) => { Val::Float(l % r) }
                            (Val::Int(l), Val::Int(r)) => { Val::Int(l % r) }
                            _ => panic!("invalid invariant")
                        }
                    }
                    Op::Div => {
                        match (left, right) {
                            (Val::Float(l), Val::Float(r)) => { Val::Float(l / r) }
                            (Val::Int(l), Val::Int(r)) => { Val::Int(l / r) }
                            _ => panic!("invalid invariant")
                        }
                    }
                    Op::Pow => {
                        match (left, right) {
                            (Val::Float(l), Val::Float(r)) => { Val::Float(l.powf(r)) }
                            (Val::Int(l), Val::Int(r)) => { Val::Int(l.pow(r as u32)) }
                            _ => panic!("invalid invariant")
                        }
                    }
                }
            }
        }
    }
}

fn unify_types(left: Val, right: Val) -> (Val, Val) {
    if left.is_int() != right.is_int() {
        (left.into_float(), right.into_float())
    } else {
        (left, right)
    }
}