use crate::prelude::*;
use crate::core::marker::PhantomData;


#[derive(Clone)]
pub struct Dict<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}
