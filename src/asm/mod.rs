use std::convert::TryFrom;
use std::marker::PhantomData;

use anyhow::Error;

use crate::asm::arch::{Asm, Elf};
use crate::asm::exec::{Arch, AsmCode, Rt};
use crate::parser::ast::{Exp, Val};

pub mod arch;
pub mod debug;
pub mod exec;
pub mod x86_64;

pub enum Fun<A: Arch> {
    Int {
        elf: Elf<fn() -> i64>,
        _a: PhantomData<A>,
    },
    Float {
        elf: Elf<fn() -> f64>,
        _a: PhantomData<A>,
    },
}

impl<A: Arch> Fun<A> {
    fn call(&self) -> Val {
        match self {
            Fun::Int { elf, .. } => {
                let fun = unsafe { elf.func() };
                Val::Int(fun())
            }
            Fun::Float { elf, .. } => {
                let fun = unsafe { elf.func() };
                Val::Float(fun())
            }
        }
    }
}

impl<A: Arch> TryFrom<Exp> for Fun<A> {
    type Error = Error;

    fn try_from(exp: Exp) -> Result<Self, Self::Error> {
        let mut asm = Asm::new();
        Ok(if exp.result_type() == Rt::Int {
            exp.to_asm::<A>(&mut asm, A::INT_RET, A::FLOAT_RET);
            A::ret(&mut asm);
            Fun::Int {
                elf: asm.prepare()?,
                _a: Default::default(),
            }
        } else {
            exp.to_asm::<A>(&mut asm, A::INT_ACC, A::FLOAT_ACC);
            Fun::Float {
                elf: asm.prepare()?,
                _a: Default::default(),
            }
        })
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use crate::asm::debug::Debug;
    use crate::asm::x86_64::X8664;
    use crate::asm::Fun;
    use crate::parser::ast::{parse_exp, Val};
    use crate::parser::lexer::Lexer;

    fn perform(input: &str, result: Val) {
        let mut lexer = Lexer::new(input);
        lexer.advance().unwrap();
        let exp = parse_exp(&mut lexer).unwrap().exp().unwrap();
        Fun::<Debug>::try_from(exp.clone());
        let fun = Fun::<X8664>::try_from(exp.clone()).unwrap();
        assert_eq!(fun.call(), result);
    }

    #[test]
    fn test_interpreter() {
        perform("13", Val::Int(13));
        //perform("-1", Val::Int(-1));
        // perform("-1.0", Val::Float(-1.0));
        // perform("13 + 13", Val::Int(13 + 13));
        // perform(" 2 * 13 + 13", Val::Int(2 * 13 + 13));
        // perform(" 2 * (13 + 13)", Val::Int(2 * (13 + 13)));
        // perform(" 2 * (13 + 13) / 2", Val::Int(2 * (13 + 13) / 2));
        // perform("(2 + 2) * 10 ^ 2", Val::Int((2 + 2) * 10i128.pow(2)));
        // perform("(2 + 2) * 10.0", Val::Float((2 + 2) as f64 * 10.0));
    }
}
