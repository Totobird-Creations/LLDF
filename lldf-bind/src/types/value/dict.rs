use crate::bind::DFOpaqueValue;
use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::core::mem::transmute_unchecked;


#[derive(Clone)]
pub struct Dict<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> Dict<T> {

    // TODO: new

    // TODO: from_arrays

}

impl<T : DFValue> Dict<T> {

    // TODO: insert

    // TODO: get

    // TODO: len

    // TODO: remove

    // TODO: clear

    #[lldf_bind_proc::dfdoc(SetVariable/GetDictKeys)]
    #[inline(always)]
    pub fn keys(&self) -> List<String> { unsafe {
        DF_ACTION__SetVariable_GetDictKeys(self.to_opaque())
    } }

    // TODO: values

    // TODO: append

}

unsafe impl<T : DFValue> DFValue for Dict<T> {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue {
        transmute_unchecked(self._opaque_type.clone())
    }
}



extern "C" {

    fn DF_ACTION__SetVariable_GetDictKeys( dict : DFOpaqueValue ) -> List<String>;

}
