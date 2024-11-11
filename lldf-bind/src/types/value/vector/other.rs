use super::*;


impl<const LANES : usize> _VectorMethods<LANES> for Vector<LANES> {

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    #[inline(always)]
    default fn new(lanes : [Float; LANES]) -> Vector<LANES> {
        Vector { inner : List::from_array(lanes) }
    }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    default unsafe fn lane_unchecked<U : Into<UInt>>(&self, lane : U) -> Float { unsafe {
        let lane = DF_ACTION__SetVariable_Specialcharplus( lane.into(), UInt::from(1usize) );
        DF_ACTION__SetVariable_GetListValue( self.inner.clone(), lane )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_GetListValue( list : List<Float>, index : UInt ) -> Float;

}


#[cfg(doc)]
impl<const LANES : usize> Vector<LANES> {

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    pub fn new(lanes : [Float; LANES]) -> Vector<LANES> { loop { /* documentation only */ } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp)]
    pub fn lane<U : Into<UInt>>(&self, lane : U) -> Float { loop { /* documentation only */ } }

}
