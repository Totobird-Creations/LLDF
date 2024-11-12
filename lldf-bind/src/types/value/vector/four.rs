use super::*;


impl Vector<4> {

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = X })]
    #[inline(always)]
    pub fn with_x<F : Into<Float>>(&self, x : F) -> Vector<4> { unsafe {
        DF_ACTION__SetVariable_CreateList( x.into(), self.y(), self.z(), self.w() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn with_y<F : Into<Float>>(&self, y : F) -> Vector<4> { unsafe {
        DF_ACTION__SetVariable_CreateList( self.x(), y.into(), self.z(), self.w() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = Z })]
    #[inline(always)]
    pub fn with_z<F : Into<Float>>(&self, z : F) -> Vector<4> { unsafe {
        DF_ACTION__SetVariable_CreateList( self.x(), self.y(), z.into(), self.w() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = W })]
    /// ##### Note
    /// - **`W` is lane 4 of the vector**
    #[inline(always)]
    pub fn with_w<F : Into<Float>>(&self, w : F) -> Vector<4> { unsafe {
        DF_ACTION__SetVariable_CreateList( self.x(), self.y(), self.z(), w.into() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.to_opaque(), UInt::from(1usize) )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.to_opaque(), UInt::from(2usize) )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Z })]
    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.to_opaque(), UInt::from(3usize) )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = W })]
    /// ##### Note
    /// - **`W` is lane 4 of the vector**
    #[inline(always)]
    pub fn w(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.to_opaque(), UInt::from(4usize) )
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_CreateList( ... ) -> Vector<4>;
    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> Float;

}
