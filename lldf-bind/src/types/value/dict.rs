use crate::prelude::*;
use crate::core::marker::PhantomData;


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

    // TODO: keys

    // TODO: values

    // TODO: append

}
