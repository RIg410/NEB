use std::{io, ptr, slice};

use anyhow::Error;
#[cfg(any(unix))]
use libc::{mmap, munmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};

pub trait Bytecode {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asm {
    bytes: Vec<u8>,
}

impl Asm {
    pub fn new() -> Asm {
        Asm {
            bytes: Vec::with_capacity(64),
        }
    }

    pub fn put(&mut self, code: &[u8]) {
        self.bytes.extend_from_slice(code);
    }

    pub fn buffer(&self) -> &[u8] {
        &self.bytes
    }

    #[cfg(any(unix))]
    pub fn prepare<T>(&self) -> Result<Elf<T>, Error> {
        if self.bytes.is_empty() {
            return Err(Error::msg("Empty buffer"));
        }

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                self.bytes.len(),
                PROT_EXEC | PROT_READ | PROT_WRITE,
                MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0,
            )
        };
        if ptr == ptr::null_mut() {
            Err(io::Error::last_os_error().into())
        } else {
            unsafe { ptr::copy(self.bytes.as_ptr(), ptr as *mut u8, self.bytes.len()) }
            Ok(Elf {
                func: ptr as *mut T,
                size: self.bytes.len(),
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Elf<T> {
    func: *mut T,
    size: usize,
}

impl<T> Elf<T> {
    pub unsafe fn func(&self) -> T
    where
        T: Copy,
    {
        *(&self.func as *const *mut T as *const T)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn bytecode(&self) -> Vec<u8> {
        Vec::from(unsafe { slice::from_raw_parts(self.func as *const u8, self.size) })
    }
}

#[cfg(any(unix))]
impl<T> Drop for Elf<T> {
    fn drop(&mut self) {
        unsafe {
            let result = munmap(self.func as *mut _, self.size);
            debug_assert!(result >= 0);
        }
    }
}


pub trait Arch {
    type IntReg: PartialEq + Eq;
    type FloatReg: PartialEq + Eq;

    const INT_RET: Self::IntReg;
    const FLOAT_RET: Self::FloatReg;

    const INT_ACC: Self::IntReg;
    const FLOAT_ACC: Self::FloatReg;

    const INT_TMP: Self::IntReg;
    const FLOAT_TMP: Self::FloatReg;

    fn movi(&mut self, from: Self::IntReg, to: Self::IntReg);
    fn movf(&mut self, from: Self::FloatReg, to: Self::FloatReg);

    fn storei(&mut self, reg: Self::IntReg, val: i64);
    fn storef(&mut self, reg: Self::FloatReg, val: f64);

    fn castf(&mut self, from: Self::IntReg, to: Self::FloatReg);

    fn addi(&mut self, op: Self::IntReg);
    fn addf(&mut self, op: Self::FloatReg);

    fn subi(&mut self, op: Self::IntReg);
    fn subf(&mut self, op: Self::FloatReg);

    fn muli(&mut self, op: Self::IntReg);
    fn mulf(&mut self, op: Self::FloatReg);

    fn modi(&mut self, op: Self::IntReg);
    fn modf(&mut self, op: Self::FloatReg);

    fn divi(&mut self, op: Self::IntReg);
    fn divf(&mut self, op: Self::FloatReg);

    fn powi(&mut self, op: Self::IntReg);
    fn powf(&mut self, op: Self::FloatReg);

    fn popi(&mut self, reg: Self::IntReg);
    fn popf(&mut self, reg: Self::FloatReg);

    fn pushli(&mut self, val: i64);
    fn pushlf(&mut self, val: f64);

    fn pushi(&mut self, reg: Self::IntReg);
    fn pushf(&mut self, reg: Self::FloatReg);

    fn ret(&mut self);
}

pub trait DebugMod {
    fn debug_mod(&mut self, debug_mod: bool);
}