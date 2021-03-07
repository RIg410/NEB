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
