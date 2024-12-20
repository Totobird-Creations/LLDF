use super::*;


#[repr(transparent)]
pub struct UInt {
    pub(crate) _opaque_type : u64
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
    #[inline(always)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_Specialcharplus(
            self,
            rhs.into()
        )
    } }
}

impl<T : Into<UInt>> Sub<T> for UInt { // TODO: Add overflow check.
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_Specialcharminus(
            self,
            rhs.into()
        )
    } }
}

impl<T : Into<UInt>> Mul<T> for UInt {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_x(
            self,
            rhs.into()
        )
    } }
}

impl<T : Into<UInt> + Clone> PartialEq<T> for UInt {
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        self._opaque_type == other.clone().into()._opaque_type
    }
}

impl<T : Into<UInt> + Clone> PartialOrd<T> for UInt {
    #[inline(always)]
    fn partial_cmp(&self, other : &T) -> Option<Ordering> {
        self._opaque_type.partial_cmp(&other.clone().into()._opaque_type)
    }
}

unsafe impl DFValue for UInt {

    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type)
    } }

    #[inline(always)]
    fn to_string(&self) -> String { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(self)
    } }

    #[inline(always)]
    fn to_text(&self) -> Text { unsafe {
        DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces(self)
    } }

}

impl Into<UInt> for UInt {
    #[inline(always)]
    fn into(self) -> UInt { self }
}


extern "C" {

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;
    fn DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces( ... ) -> Text;

    fn DF_ACTION__SetVariable_Specialcharplus( left : UInt, right : UInt ) -> UInt;
    fn DF_ACTION__SetVariable_Specialcharminus( left : UInt, right : UInt ) -> UInt;
    fn DF_ACTION__SetVariable_x( left : UInt, right : UInt ) -> UInt;

}
