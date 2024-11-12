use super::*;


impl Vector<2> {

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = X })]
    #[inline(always)]
    pub fn with_x<F : Into<Float>>(&self, x : F) -> Vector<2> { unsafe {
        DF_ACTION__SetVariable_CreateList( x.into(), self.y() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn with_y<F : Into<Float>>(&self, y : F) -> Vector<2> { unsafe {
        DF_ACTION__SetVariable_CreateList( self.x(), y.into() )
    } }

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

    fn DF_ACTION__SetVariable_CreateList( ... ) -> Vector<2>;
    fn DF_ACTION__SetVariable_GetListValue( list : Vector<2>, index : UInt ) -> Float;

}
