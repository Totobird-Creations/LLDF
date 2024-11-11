use super::*;


impl Vector<1> {

    // TODO: with_x

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(1usize) )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<1>, index : UInt ) -> Float;

}
