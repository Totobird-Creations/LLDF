use super::*;


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
        DF_ACTION__SetVariable_Specialcharplus(
            self,
            rhs.into()
        )
    } }
}

unsafe impl DFValue for UInt {
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__UInt_Opaque(self)
    } }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__UInt_Opaque( from : UInt ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_Specialcharplus( left : UInt, right : UInt ) -> UInt;

}
