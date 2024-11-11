use super::*;


impl Vector<2> {

    // TODO: with_x

    // TODO: with_y

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(1usize) )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(2usize) )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<2>, index : UInt ) -> Float;

}
