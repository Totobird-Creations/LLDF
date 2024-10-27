use crate::prelude::*;
use crate::core::marker::PhantomData;


#[derive(Clone, Copy)]
pub struct Dict<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}


unsafe impl<T : DFValue> DFValue for Dict<T> {}
