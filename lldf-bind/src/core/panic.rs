use super::prelude::*;


#[lang = "panic_location"]
#[derive(Copy, Clone)]
pub struct Location<'l> {
    file : &'l str,
    line : u32,
    col  : u32
}
impl Location<'_> {

    #[inline(always)]
    pub fn file(&self) -> &str { self.file }

    #[inline(always)]
    pub fn line(&self) -> u32 { self.line }

    #[inline(always)]
    pub fn col(&self) -> u32 { self.col }

}
