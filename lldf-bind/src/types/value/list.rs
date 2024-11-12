use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::bind::DFOpaqueValue;
use core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct List<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> Clone for List<T> {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_Specialcharequals(transmute_unchecked(self._opaque_type.clone())))
    } }
}

impl<T : DFValue> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/CreateList)]
    #[inline(always)]
    pub fn new() -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_CreateList())
    } }

    #[inline(always)]
    pub fn from_array<const N : usize>(array : [T; N]) -> Self { unsafe { // FIXME when arrays are made GEP-compatible
        transmute_unchecked(DF_TRANSMUTE__Array_Opaque(array))
    } }

}

impl<T : DFValue> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/AppendValue)]
    #[inline(always)]
    pub fn push(&mut self, value : T) -> () { unsafe {
        DF_ACTION__SetVariable_AppendValue(self.to_opaque(), value.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/AppendList)]
    #[inline(always)]
    pub fn append(&mut self, other : Self) -> () { unsafe {
        DF_ACTION__SetVariable_AppendList(self.to_opaque(), other.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetListValue)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    pub unsafe fn get_unchecked<U : Into<UInt>>(&self, index : U) -> T { unsafe {
        let index = DF_ACTION__SetVariable_Specialcharplus( index.into(), UInt::from(1usize) );
        transmute_unchecked(DF_ACTION__SetVariable_GetListValue(self.to_opaque(), index))
    } }

    // TODO: get (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/PopListValue)]
    /// ##### Unsafe
    /// - **Does not do a non-empty check**
    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> T { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_PopListValue(self.to_opaque()))
    } }

    // TODO: pop (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/PopListValue)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    pub unsafe fn remove_unchecked<U : Into<UInt>>(&mut self, index : U) -> T { unsafe {
        let index = DF_ACTION__SetVariable_Specialcharplus( index.into(), UInt::from(1usize) );
        transmute_unchecked(DF_ACTION__SetVariable_PopListValue(self.to_opaque(), index))
    } }

    // TODO: remove (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/SetListValue)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    pub unsafe fn set_unchecked<U : Into<UInt>>(&mut self, index : U, value : T) -> () { unsafe {
        let index = DF_ACTION__SetVariable_Specialcharplus( index.into(), UInt::from(1usize) );
        DF_ACTION__SetVariable_SetListValue(self.to_opaque(), index, value.to_opaque());
    } }

    // TODO: set (with bounds check)

    // TODO: index_of

    #[lldf_bind_proc::dfdoc(SetVariable/ListLength)]
    #[inline(always)]
    pub fn len(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_ListLength(self.to_opaque())
    } }

    // TODO: insert_unchecked

    // TODO: insert (with bounds check)

    // TODO: erase (remove by equality)

    #[inline(always)]
    pub fn clear(&mut self) -> () { unsafe { // TODO: test this to make sure it works properly
        *self = transmute_unchecked(DF_ACTION__SetVariable_CreateList())
    } }

    // TODO: splice

    // TODO: map

    // TODO: dedup

    // TODO: dedup_by

    // TODO: sorted_by

    #[lldf_bind_proc::dfdoc(SetVariable/ReverseList)]
    #[inline(always)]
    pub fn reversed(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_ReverseList(self.to_opaque()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/RandomizeList)]
    #[inline(always)]
    pub fn shuffled(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_RandomizeList(self.to_opaque()))
    } }

    // TODO: random (random element)

}

impl<T : DFSortableValue> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/SortList)]
    #[inline(always)]
    pub fn sorted(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_SortList(self.to_opaque()))
    } }

}

impl<T : DFValue> List<List<T>> {

    #[lldf_bind_proc::dfdoc(SetVariable/FlattenList)]
    #[inline(always)]
    pub fn flattened(&self) -> List<T> { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_FlattenList(self.to_opaque()))
    } }

}

impl List<String> {

    #[lldf_bind_proc::dfdoc(SetVariable/JoinString)]
    #[inline(always)]
    pub fn join<S : Into<String>>(&self, delimiter : S) -> String { unsafe {
        DF_ACTION__SetVariable_JoinString(self.to_opaque(), delimiter.into())
    } }

}

unsafe impl<T : DFValue> DFValue for List<T> {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue {
        transmute_unchecked(self._opaque_type.clone())
    }
}





extern "C" {

    fn DF_TRANSMUTE__Array_Opaque( ... ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_Specialcharequals( from : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_CreateList( ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_AppendValue( list : DFOpaqueValue, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_AppendList( list : DFOpaqueValue, from : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_PopListValue( list : DFOpaqueValue, ... ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_SetListValue( list : DFOpaqueValue, index : UInt, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_ListLength( list : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_ReverseList( list : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_RandomizeList( list : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_FlattenList( list : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_SortList( list : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_JoinString( list : DFOpaqueValue, delimiter : String ) -> String;

}
