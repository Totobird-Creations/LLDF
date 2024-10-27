use super::clone::Clone;
use super::macros::derive;
use super::marker::{ Copy, Sized };


extern "rust-intrinsic" {
    #[rustc_nounwind]
    pub fn transmute_unchecked<Src, Dst>(src: Src) -> Dst;
}


#[lang = "maybe_uninit"]
//#[derive(Clone, Copy)]
#[repr(transparent)]
pub union MaybeUninit<T> {
    uninit : (),
    value : ManuallyDrop<T>
}
impl<T> MaybeUninit<T> {

    #[must_use]
    #[inline(always)]
    #[rustc_diagnostic_item = "maybe_uninit_uninit"]
    pub const fn uninit() -> Self { Self { uninit : () } }

    #[inline(always)]
    #[rustc_diagnostic_item = "assume_init"]
    pub const unsafe fn assume_init(self) -> T { unsafe {
        self.value.value
    } }

}


#[lang = "manually_drop"]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ManuallyDrop<T : ?Sized> {
    value : T
}
