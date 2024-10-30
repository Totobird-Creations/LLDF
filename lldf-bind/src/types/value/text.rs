use crate::prelude::*;
use crate::core::mem::MaybeUninit;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Text {
    _opaque_type : u8
}

impl<T : DFValue> From<T> for Text {
    #[inline(always)]
    fn from(value : T) -> Text { unsafe {
        let mut out : MaybeUninit<Text> = MaybeUninit::uninit();
        DF_ACTION__SetVariable_Text(
            (&mut out) as *mut _ as *mut Text,
            &value as *const _ as *const DFOpaqueValue
        );
        out.assume_init()
    } }
}

unsafe impl DFValue for Text {}


extern "C" {

    fn DF_ACTION__SetVariable_Text( dest : *mut Text, from : *const DFOpaqueValue ) -> ();

}
