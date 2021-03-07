pub trait AsmCode {
    fn to_asm(&self, arch: impl Arch) -> Code;
}

pub struct Code {

}

pub trait Arch {
    
}