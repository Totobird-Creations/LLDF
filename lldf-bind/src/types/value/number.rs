use crate::prelude::*;
use crate::core::ops::Add;
use crate::core::mem::MaybeUninit;
use crate::bind::DFOpaqueValue;


#[repr(transparent)]
pub struct UInt {
    _opaque_type : u8
}
impl Clone for UInt { fn clone(&self) -> Self { *self } }
impl Copy for UInt {}

impl From<bool  > for UInt { #[inline(always)] fn from(value : bool  ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<u8    > for UInt { #[inline(always)] fn from(value : u8    ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<u16   > for UInt { #[inline(always)] fn from(value : u16   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<u32   > for UInt { #[inline(always)] fn from(value : u32   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<u64   > for UInt { #[inline(always)] fn from(value : u64   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<u128  > for UInt { #[inline(always)] fn from(value : u128  ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<usize > for UInt { #[inline(always)] fn from(value : usize ) -> Self { unsafe { *(&value as *const _ as *const _) } } }

impl<T : Into<UInt>> Add<T> for UInt {
    type Output = Self;
    #[inline(never)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        let mut out : MaybeUninit<UInt> = MaybeUninit::uninit();
        DF_ACTION__SetVariable_Specialcharplus(
            (&mut out) as *mut _ as *mut _,
            &self as *const _ as *const _,
            &rhs as *const _ as *const _
        );
        out.assume_init()
    } }
}

unsafe impl DFValue for UInt {}


pub struct Int {
    _opaque_type : u8
}
impl Clone for Int { fn clone(&self) -> Self { *self } }
impl Copy for Int {}

impl From<i8    > for Int { #[inline(always)] fn from(value : i8    ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i16   > for Int { #[inline(always)] fn from(value : i16   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i32   > for Int { #[inline(always)] fn from(value : i32   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i64   > for Int { #[inline(always)] fn from(value : i64   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i128  > for Int { #[inline(always)] fn from(value : i128  ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<isize > for Int { #[inline(always)] fn from(value : isize ) -> Self { unsafe { *(&value as *const _ as *const _) } } }

unsafe impl DFValue for Int {}


pub struct Float {
    _opaque_type : u8
}
impl Clone for Float { fn clone(&self) -> Self { *self } }
impl Copy for Float {}

impl From<f32> for Float { #[inline(always)] fn from(value : f32) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<f64> for Float { #[inline(always)] fn from(value : f64) -> Self { unsafe { *(&value as *const _ as *const _) } } }

unsafe impl DFValue for Float {}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( dest : *mut DFOpaqueValue, left : *const DFOpaqueValue, right : *const DFOpaqueValue ) -> ();

}
