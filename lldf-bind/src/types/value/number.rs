use crate::prelude::*;
use crate::core::ops::Add;


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UInt {
    value : u64
}

impl Into<UInt> for u8 { #[inline(always)] fn into(self) -> UInt { UInt { value : self as u64 } } }
impl Into<UInt> for u16 { #[inline(always)] fn into(self) -> UInt { UInt { value : self as u64 } } }
impl Into<UInt> for u32 { #[inline(always)] fn into(self) -> UInt { UInt { value : self as u64 } } }
impl Into<UInt> for u64 { #[inline(always)] fn into(self) -> UInt { UInt { value : self } } }
impl Into<UInt> for u128 { #[inline(always)] fn into(self) -> UInt { UInt { value : self as u64 } } }
impl Into<UInt> for usize { #[inline(always)] fn into(self) -> UInt { UInt { value : self as u64 } } }

impl<T : Into<UInt>> Add<T> for UInt {
    type Output = Self;
    fn add(self, rhs : T) -> Self::Output { Self { value : self.value + rhs.into().value } }
}

unsafe impl DFValue for UInt {}


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Int {
    value : i64
}

impl Into<Int> for i8 { #[inline(always)] fn into(self) -> Int { Int { value : self as i64 } } }
impl Into<Int> for i16 { #[inline(always)] fn into(self) -> Int { Int { value : self as i64 } } }
impl Into<Int> for i32 { #[inline(always)] fn into(self) -> Int { Int { value : self as i64 } } }
impl Into<Int> for i64 { #[inline(always)] fn into(self) -> Int { Int { value : self } } }
impl Into<Int> for i128 { #[inline(always)] fn into(self) -> Int { Int { value : self as i64 } } }
impl Into<Int> for isize { #[inline(always)] fn into(self) -> Int { Int { value : self as i64 } } }

unsafe impl DFValue for Int {}


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Float {
    value : f64
}

impl Into<Float> for f32 { #[inline(always)] fn into(self) -> Float { Float { value : self as f64 } } }
impl Into<Float> for f64 { #[inline(always)] fn into(self) -> Float { Float { value : self } } }

unsafe impl DFValue for Float {}
