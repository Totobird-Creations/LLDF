use super::*;


impl Vector<4> {

    // TODO: with_x

    // TODO: with_y

    // TODO: with_z

    // TODO: with_w

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

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Z })]
    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(3usize) )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = W })]
    #[inline(always)]
    pub fn w(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(4usize) )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<4>, index : UInt ) -> Float;

}
