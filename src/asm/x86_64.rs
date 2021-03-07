use std::fmt::{Display, Formatter};
use std::fmt;

use anyhow::{anyhow, Error, Result};

use crate::asm::arch::{Asm, Arch, DebugMod};

pub struct X8664 {
    asm: Asm,
    debug: bool,
}

impl X8664 {
    fn put(&mut self, bytes: &[u8]) {
        self.asm.put(bytes);
    }

    fn dbg(&self, f: impl Fn()) {
        if self.debug {
            f()
        }
    }
}

impl Arch for X8664 {
    type IntReg = IntReg;
    type FloatReg = FloatReg;

    const INT_RET: Self::IntReg = IntReg::RAX;
    const FLOAT_RET: Self::FloatReg = FloatReg::XMM0;

    const INT_ACC: Self::IntReg = IntReg::RAX;
    const FLOAT_ACC: Self::FloatReg = FloatReg::XMM0;

    const INT_TMP: Self::IntReg = IntReg::RCX;
    const FLOAT_TMP: Self::FloatReg = FloatReg::XMM1;

    fn movi(&mut self, from: Self::IntReg, to: Self::IntReg) {
        self.dbg(|| println!("movi {}, {}", to, from));

        if from == to {
            return;
        }
        self.put(&[0x48, 0x89]);

        if from == IntReg::RAX && to == IntReg::RCX {
            self.put(&[0xc1]);
            return;
        }

        if from == IntReg::RCX && to == IntReg::RAX {
            self.put(&[0xc8]);
        }
    }

    fn movf(&mut self, from: Self::FloatReg, to: Self::FloatReg) {
        self.dbg(|| println!("movf {}, {}", to, from));
        unimplemented!()
    }

    fn storei(&mut self, reg: Self::IntReg, val: i64) {
        self.dbg(|| println!("movi {}, {}", reg, val));

        self.put(&[0x48]);
        match reg {
            IntReg::RAX => self.put(&[0xb8]),
            IntReg::RCX => self.put(&[0xb9]),
        }
        self.put(&val.to_le_bytes());
    }

    fn storef(&mut self, reg: Self::FloatReg, val: f64) {
        self.dbg(|| println!("movf {}, {}", reg, val));
        unimplemented!()
    }

    fn castf(&mut self, from: Self::IntReg, to: Self::FloatReg) {
        self.dbg(|| println!("movc {}, {}", to, from));
        unimplemented!()
    }

    fn addi(&mut self, op: Self::IntReg) {
        self.dbg(|| println!("addi {}, {}", Self::INT_ACC, op));

        self.put(&[0x48, 0x01]);
        match op {
            IntReg::RAX => self.put(&[0xc0]),
            IntReg::RCX => self.put(&[0xc8]),
        }
    }

    fn addf(&mut self, op: Self::FloatReg) {
        self.dbg(|| println!("addi {}, {}", Self::FLOAT_ACC, op));
        unimplemented!()
    }

    fn subi(&mut self, op: Self::IntReg) {
        self.dbg(|| println!("subi {}, {}", Self::INT_ACC, op));

        unimplemented!()
    }

    fn subf(&mut self, op: Self::FloatReg) {
        self.dbg(|| println!("subf {}, {}", Self::FLOAT_ACC, op));

        unimplemented!()
    }

    fn muli(&mut self, op: Self::IntReg) {
        self.dbg(|| println!("muli {}, {}", Self::INT_ACC, op));

        self.put(&[0x48, 0xf7]);
        match op {
            IntReg::RAX => self.put(&[0xe8]),
            IntReg::RCX => self.put(&[0xe9]),
        }
    }

    fn mulf(&mut self, op: Self::FloatReg) {
        self.dbg(|| println!("mulf {}, {}", Self::FLOAT_ACC, op));

        unimplemented!()
    }

    fn modi(&mut self, op: Self::IntReg) {
        unimplemented!()
    }

    fn modf(&mut self, op: Self::FloatReg) {
        unimplemented!()
    }

    fn divi(&mut self, op: Self::IntReg) {
        self.dbg(|| println!("divi {}, {}", Self::INT_ACC, op));

        self.put(&[0x48, 0xf7]);
        match op {
            IntReg::RAX => self.put(&[0xf8]),
            IntReg::RCX => self.put(&[0xf9]),
        }
    }

    fn divf(&mut self, op: Self::FloatReg) {
        self.dbg(|| println!("divf {}, {}", Self::INT_ACC, op));
        unimplemented!()
    }

    fn powi(&mut self, op: Self::IntReg) {
        unimplemented!()
    }

    fn powf(&mut self, op: Self::FloatReg) {
        unimplemented!()
    }

    fn popi(&mut self, reg: Self::IntReg) {
        unimplemented!()
    }

    fn popf(&mut self, reg: Self::FloatReg) {
        unimplemented!()
    }

    fn pushli(&mut self, val: i64) {
        unimplemented!()
    }

    fn pushlf(&mut self, val: f64) {
        unimplemented!()
    }

    fn pushi(&mut self, reg: Self::IntReg) {
        unimplemented!()
    }

    fn pushf(&mut self, reg: Self::FloatReg) {
        unimplemented!()
    }

    fn ret(&mut self) {
        self.dbg(|| println!("ret"));
        self.put(&[0xc3]);
    }
}

impl DebugMod for X8664 {
    fn debug_mod(&mut self, debug_mod: bool) {
        self.debug = debug_mod;
    }
}

impl Default for X8664 {
    fn default() -> Self {
        X8664 {
            asm: Asm::new(),
            debug: false,
        }
    }
}

impl Into<Asm> for X8664 {
    fn into(self) -> Asm {
        self.asm
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntReg {
    RAX,
    RCX,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FloatReg {
    XMM0,
    XMM1,
}

impl Display for IntReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IntReg::RAX => write!(f, "rax"),
            IntReg::RCX => write!(f, "rcx"),
        }
    }
}

impl Display for FloatReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FloatReg::XMM0 => write!(f, "xmm0"),
            FloatReg::XMM1 => write!(f, "xmm1"),
        }
    }
}