use super::clone::Clone;
use super::macros::derive;
use super::marker::{ Copy, Sized };


#[rustc_intrinsic]
#[rustc_intrinsic_must_be_overridden]
pub const unsafe fn transmute_unchecked<Src, Dst>(_src : Src) -> Dst { loop {} }


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
    pub const fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }

    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }

    #[inline(always)]
    #[rustc_diagnostic_item = "assume_init"]
    #[track_caller]
    pub const unsafe fn assume_init(self) -> T { unsafe {
        self.value.value
    } }

    #[inline(always)]
    pub const unsafe fn assume_init_ref(&self) -> &T { unsafe {
        &*self.as_ptr()
    } }

    #[inline(always)]
    pub const unsafe fn assume_init_mut(&mut self) -> &mut T { unsafe {
        &mut*self.as_mut_ptr()
    } }

}
impl<T : Clone> Clone for MaybeUninit<T> {
    fn clone(&self) -> Self { Self { value : unsafe { self.value.clone() } } }
}


#[lang = "manually_drop"]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ManuallyDrop<T : ?Sized> {
    value : T
}

impl<T> ManuallyDrop<T> {

    #[inline(always)]
    pub const fn new(value: T) -> ManuallyDrop<T> {
        ManuallyDrop { value }
    }

}
