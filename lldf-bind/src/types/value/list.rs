use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::bind::DFOpaqueValue;
use core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct List<T : Clone> {
    pub(crate) _opaque_type : u64,
    _ph : PhantomData<T>
}

impl<T : Clone> Clone for List<T> {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(DF_ACTION__SetVariable_Specialcharequals(transmute_unchecked(self._opaque_type))) } }
}

impl<T : Clone> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/CreateList)]
    #[inline(always)]
    pub fn new() -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_CreateList())
    } }

    #[inline(always)]
    pub fn from_array<const N : usize>(array : [T; N]) -> Self { unsafe { // FIXME when arrays are made GEP-compatible
        transmute_unchecked(DF_TRANSMUTE__Opaque(array))
    } }

}

impl<T : Clone> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/AppendValue)]
    #[inline(always)]
    pub fn push(&mut self, value : T) -> () { unsafe {
        DF_ACTION__SetVariable_AppendValue(self as *mut _ as *mut _, DF_TRANSMUTE__Opaque(value))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/AppendList)]
    #[inline(always)]
    pub fn append(&mut self, other : Self) -> () { unsafe {
        DF_ACTION__SetVariable_AppendList(self as *mut _ as *mut _, DF_TRANSMUTE__Opaque(other))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetListValue)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    pub unsafe fn get_unchecked<U : Into<UInt>>(&self, index : U) -> T { unsafe {
        let index = DF_ACTION__SetVariable_Specialcharplus( index.into(), UInt::from(1usize) );
        transmute_unchecked(DF_ACTION__SetVariable_GetListValue(self as *const _ as *const _, index))
    } }

    // TODO: get (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/PopListValue)]
    /// ##### Unsafe
    /// - **Does not do a non-empty check**
    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> T { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_PopListValue(self as *mut _ as *mut _))
    } }

    // TODO: pop (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/PopListValue)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    pub unsafe fn remove_unchecked<U : Into<UInt>>(&mut self, index : U) -> T { unsafe {
        let index = DF_ACTION__SetVariable_Specialcharplus( index.into(), UInt::from(1usize) );
        transmute_unchecked(DF_ACTION__SetVariable_PopListValue(self as *mut _ as *mut _, index))
    } }

    // TODO: remove (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/SetListValue)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    pub unsafe fn set_unchecked<U : Into<UInt>>(&mut self, index : U, value : T) -> () { unsafe {
        let index = DF_ACTION__SetVariable_Specialcharplus( index.into(), UInt::from(1usize) );
        DF_ACTION__SetVariable_SetListValue(self as *mut _ as *mut _, index, DF_TRANSMUTE__Opaque(value));
    } }

    // TODO: set (with bounds check)

    // TODO: index_of

    #[lldf_bind_proc::dfdoc(SetVariable/ListLength)]
    #[inline(always)]
    pub fn len(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_ListLength(self as *const _ as *const _)
    } }

    // TODO: insert_unchecked

    // TODO: insert (with bounds check)

    #[lldf_bind_proc::dfdoc(SetVariable/RemoveListValue)]
    #[inline(always)]
    pub fn erase(&mut self, value : &T) -> () { unsafe { // TODO: Only on non-list non-dict elements.
        DF_ACTION__SetVariable_RemoveListValue_ItemsToRemove_AllMatches(self as *mut _ as *mut _, DF_TRANSMUTE__Opaque(value));
    } }

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
        transmute_unchecked(DF_ACTION__SetVariable_ReverseList(self as *const _ as *const _))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/RandomizeList)]
    #[inline(always)]
    pub fn shuffled(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_RandomizeList(self as *const _ as *const _))
    } }

    // TODO: random (random element)

}

impl<T : DFSortableValue> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/SortList)]
    #[inline(always)]
    pub fn sorted(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_SortList(self as *const _ as *const _))
    } }

}

impl<T : DFValue> List<List<T>> {

    #[lldf_bind_proc::dfdoc(SetVariable/FlattenList)]
    #[inline(always)]
    pub fn flattened(&self) -> List<T> { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_FlattenList(self as *const _ as *const _))
    } }

}

impl List<String> {

    #[lldf_bind_proc::dfdoc(SetVariable/JoinString)]
    #[inline(always)]
    pub fn join<S : Into<String>>(&self, delimiter : S) -> String { unsafe {
        DF_ACTION__SetVariable_JoinString(self as *const _ as *const _, delimiter.into())
    } }

}

unsafe impl<T : DFValue> DFValue for List<T> {

    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { transmute_unchecked(self._opaque_type) }

    #[inline(always)]
    fn to_string(&self) -> String { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(String::new(), self)
    } }

    #[inline(always)]
    fn to_text(&self) -> Text { unsafe {
        DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces(Text::new(), self)
    } }

}





extern "C" {

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;
    fn DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces( ... ) -> Text;

    fn DF_TRANSMUTE__Opaque( ... ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_Specialcharequals( from : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_CreateList( ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_AppendValue( list : *mut DFOpaqueValue, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_AppendList( list : *mut DFOpaqueValue, from : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_GetListValue( list : *const DFOpaqueValue, index : UInt ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_PopListValue( list : *mut DFOpaqueValue, ... ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_SetListValue( list : *mut DFOpaqueValue, index : UInt, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_ListLength( list : *const DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_RemoveListValue_ItemsToRemove_AllMatches( list : *mut DFOpaqueValue, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_ReverseList( list : *const DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_RandomizeList( list : *const DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_FlattenList( list : *const DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_SortList( list : *const DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_JoinString( list : *const DFOpaqueValue, delimiter : String ) -> String;

}
