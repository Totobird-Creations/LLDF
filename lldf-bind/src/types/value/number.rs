use crate::prelude::*;
use crate::core::ops::Add;
use crate::core::mem::MaybeUninit;
use crate::bind::DFOpaqueValue;


pub union UInt {
    bool  : bool,
    u8    : u8,
    u16   : u16,
    u32   : u32,
    u64   : u64,
    u128  : u128,
    usize : usize
}
impl Clone for UInt { fn clone(&self) -> Self { *self } }
impl Copy for UInt {}

impl From<bool  > for UInt { #[inline(always)] fn from(value : bool  ) -> Self { UInt { bool  : value } } }
impl From<u8    > for UInt { #[inline(always)] fn from(value : u8    ) -> Self { UInt { u8    : value } } }
impl From<u16   > for UInt { #[inline(always)] fn from(value : u16   ) -> Self { UInt { u16   : value } } }
impl From<u32   > for UInt { #[inline(always)] fn from(value : u32   ) -> Self { UInt { u32   : value } } }
impl From<u64   > for UInt { #[inline(always)] fn from(value : u64   ) -> Self { UInt { u64   : value } } }
impl From<u128  > for UInt { #[inline(always)] fn from(value : u128  ) -> Self { UInt { u128  : value } } }
impl From<usize > for UInt { #[inline(always)] fn from(value : usize ) -> Self { UInt { usize : value } } }

impl<T : Into<UInt>> Add<T> for UInt {
    type Output = Self;
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


pub union Int {
    i8    : i8,
    i16   : i16,
    i32   : i32,
    i64   : i64,
    i128  : i128,
    isize : isize
}
impl Clone for Int { fn clone(&self) -> Self { *self } }
impl Copy for Int {}

impl From<i8    > for Int { #[inline(always)] fn from(value : i8    ) -> Self { Int { i8    : value } } }
impl From<i16   > for Int { #[inline(always)] fn from(value : i16   ) -> Self { Int { i16   : value } } }
impl From<i32   > for Int { #[inline(always)] fn from(value : i32   ) -> Self { Int { i32   : value } } }
impl From<i64   > for Int { #[inline(always)] fn from(value : i64   ) -> Self { Int { i64   : value } } }
impl From<i128  > for Int { #[inline(always)] fn from(value : i128  ) -> Self { Int { i128  : value } } }
impl From<isize > for Int { #[inline(always)] fn from(value : isize ) -> Self { Int { isize : value } } }

unsafe impl DFValue for Int {}


pub union Float {
    f32 : f32,
    f64 : f64
}
impl Clone for Float { fn clone(&self) -> Self { *self } }
impl Copy for Float {}

impl From<f32> for Float { #[inline(always)] fn from(value : f32) -> Self { Float { f32 : value } } }
impl From<f64> for Float { #[inline(always)] fn from(value : f64) -> Self { Float { f64 : value } } }

unsafe impl DFValue for Float {}


extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( dest : *mut DFOpaqueValue, left : *const DFOpaqueValue, right : *const DFOpaqueValue ) -> ();

}
