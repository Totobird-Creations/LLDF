use super::*;


impl Vector<1> {

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = X })]
    #[inline(always)]
    pub fn with_x<F : Into<Float>>(&self, x : F) -> Vector<1> { unsafe {
        DF_ACTION__SetVariable_CreateList( x.into() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(1usize) )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_CreateList( ... ) -> Vector<1>;

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<1>, index : UInt ) -> Float;

}
