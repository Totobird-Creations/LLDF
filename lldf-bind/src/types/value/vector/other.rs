use super::*;


impl<const LANES : usize> _VectorMethods<LANES> for Vector<LANES> {

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    #[inline(always)]
    default fn new(lanes : [Float; LANES]) -> Vector<LANES> {
        Vector { inner : List::from_array(lanes) }
    }

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    #[inline(always)]
    default fn splat<F : Into<Float>>(lanes : F) -> Vector<LANES> { unsafe {
        let lanes  = lanes.into();
        let vector = DF_ACTION__SetVariable_CreateList();
        extern "C" {
            fn DF_ACTION__Repeat_Multiple( count : UInt ) -> ();
        }
        DF_ACTION__Repeat_Multiple(UInt::from(LANES)); DF_BRACKET__Repeat_Open();
            DF_ACTION__SetVariable_AppendValue(vector.to_opaque(), lanes);
        DF_BRACKET__Repeat_Close();
        transmute_unchecked(vector)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    default unsafe fn lane_unchecked<U : Into<UInt>>(&self, lane : U) -> Float { unsafe {
        let lane = DF_ACTION__SetVariable_Specialcharplus( lane.into(), UInt::from(1usize) );
        DF_ACTION__SetVariable_GetListValue( self.inner.to_opaque(), lane )
    } }


    #[doc(hidden)]
    #[inline(always)]
    default fn clone(&self) -> Self { Self { inner : self.inner.clone() } }

    #[doc(hidden)]
    #[inline(always)]
    default fn to_string(&self) -> String {
        String::from("<") + unsafe{ DF_ACTION__SetVariable_JoinString(&self.inner as *const _ as *const _, String::from(", ")) } + String::from(">")
    }

    #[doc(hidden)]
    #[inline(always)]
    default fn to_text(&self) -> Text {
        Text::from("<") + Text::from(unsafe{ DF_ACTION__SetVariable_JoinString(&self.inner as *const _ as *const _, String::from(", ")) }) + Text::from(">")
    }

}


extern "C" {

    fn DF_ACTION__SetVariable_JoinString( list : *const DFOpaqueValue, delimiter : String ) -> String;

    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_CreateList( ... ) -> List<Float>;
    fn DF_ACTION__SetVariable_AppendValue( list : DFOpaqueValue, value : Float ) -> ();
    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> Float;

    fn DF_BRACKET__Repeat_Open() -> ();
    fn DF_BRACKET__Repeat_Close() -> ();

}


#[cfg(doc)] // TODO: fix this.
impl<const LANES : usize> Vector<LANES> {

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    pub fn new(lanes : [Float; LANES]) -> Vector<LANES> { loop { /* documentation only */ } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp)]
    pub fn lane<U : Into<UInt>>(&self, lane : U) -> Float { loop { /* documentation only */ } }

}
