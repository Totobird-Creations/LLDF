use super::*;


impl<const LANES : usize> _VectorMethods<LANES> for Vector<LANES> {

    #[inline(always)]
    default fn new(lanes : [Float; LANES]) -> Vector<LANES> {
        Vector { inner : List::from_array(lanes) }
    }

    #[inline(always)]
    default fn lane<U : Into<UInt>>(&self, lane : U) -> Float { unsafe { // TODO: Add a bounds check.
        let lane = DF_ACTION__SetVariable_Specialcharplus( lane.into(), UInt::from(1usize) );
        DF_ACTION__SetVariable_GetListValue( self.inner.clone(), lane )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_GetListValue( list : List<Float>, index : UInt ) -> Float;

}

