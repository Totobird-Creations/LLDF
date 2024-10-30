use crate::prelude::*;
use crate::core::mem::{ transmute_unchecked, MaybeUninit };
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct String {
    _opaque_type : u8
}

impl<T : DFValue> From<T> for String {
    #[inline(always)]
    fn from(value : T) -> String { unsafe {
        let mut out : MaybeUninit<String> = MaybeUninit::uninit();
        DF_ACTION__SetVariable_String(
            (&mut out) as *mut _ as *mut String,
            &value as *const _ as *const DFOpaqueValue
        );
        out.assume_init()
    } }
}

impl From<&str> for String {
    #[inline(always)]
    fn from(value : &str) -> String { unsafe {
        transmute_unchecked::<_, &String>(&value).clone()
    } }
}

unsafe impl DFValue for String {}


extern "C" {

    fn DF_ACTION__SetVariable_String( dest : *mut String, from : *const DFOpaqueValue ) -> ();

}
