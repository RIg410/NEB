use std::convert::TryFrom;
use std::marker::PhantomData;

use anyhow::Error;

use crate::asm::arch::{Arch, Asm, DebugMod, Elf};
use crate::asm::exec::{AsmCode, Rt};
use crate::parser::ast::{Exp, Val};

pub mod arch;
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
    pub fn call(&self) -> Val {
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
    pub fn bytecode(&self) -> Vec<u8> {
        match self {
            Fun::Int { elf, _a } => elf.bytecode(),
            Fun::Float { elf, _a } => elf.bytecode(),
        }
    }
}

impl<A> TryFrom<Exp> for Fun<A> where A: Arch + Default + Into<Asm> {
    type Error = Error;

    fn try_from(exp: Exp) -> Result<Self, Self::Error> {
        Self::try_from((exp, A::default()))
    }
}

impl<A> TryFrom<(Exp, A)> for Fun<A> where A: Arch + Into<Asm> {
    type Error = Error;

    fn try_from((exp, mut arch): (Exp, A)) -> Result<Self, Self::Error> {
        Ok(if exp.result_type() == Rt::Int {
            exp.to_asm::<A>(&mut arch, A::INT_RET, A::FLOAT_RET);
            arch.ret();
            let asm = arch.into();
            Fun::Int {
                elf: asm.prepare()?,
                _a: Default::default(),
            }
        } else {
            exp.to_asm::<A>(&mut arch, A::INT_ACC, A::FLOAT_ACC);
            arch.ret();
            let asm = arch.into();
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

    use crate::asm::arch::{Asm, DebugMod};
    use crate::asm::Fun;
    use crate::asm::x86_64::X8664;
    use crate::parser::ast::{parse_exp, Val};
    use crate::parser::lexer::Lexer;

    fn perform(input: &str, result: Val) {
        let mut lexer = Lexer::new(input);
        lexer.advance().unwrap();
        let exp = parse_exp(&mut lexer).unwrap().exp().unwrap();
        let mut arch = X8664::default();
        arch.debug_mod(true);

        let fun = Fun::<X8664>::try_from((exp, arch)).unwrap();
        assert_eq!(fun.call(), result);
    }

    #[test]
    fn test_interpreter() {
        // perform("13", Val::Int(13));
        // perform("-1", Val::Int(-1));
        // perform("-1.0", Val::Float(-1.0));
        //perform("13 + 13", Val::Int(13 + 13));
        // perform(" 2 * 13 + 13", Val::Int(2 * 13 + 13));
        // perform(" 2 * (13 + 13)", Val::Int(2 * (13 + 13)));
        perform(" 2 * (13 + 13) / 2", Val::Int(2 * (13 + 13) / 2));
        // perform("(2 + 2) * 10 ^ 2", Val::Int((2 + 2) * 10i128.pow(2)));
        // perform("(2 + 2) * 10.0", Val::Float((2 + 2) as f64 * 10.0));
    }

    #[test]
    fn tes() {
        let mut asm = Asm::new();
        asm.put(&[0x48, 0xB9, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F, 0x48, 0x89, 0xC8, 0xC3]);
        let fun = asm.prepare::<fn() -> i64>().unwrap();
        let f = unsafe { fun.func() };
        dbg!(f());
    }
}
