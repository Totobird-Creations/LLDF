use super::*;


impl Vector<3> {

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = X })]
    #[inline(always)]
    pub fn with_x<F : Into<Float>>(&self, x : F) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_SetVectorComp_Component_X(self.to_opaque(), x.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn with_y<F : Into<Float>>(&self, y : F) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_SetVectorComp_Component_Y(self.to_opaque(), y.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetVectorComp { Component = Z })]
    #[inline(always)]
    pub fn with_z<F : Into<Float>>(&self, z : F) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_SetVectorComp_Component_Z(self.to_opaque(), z.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_X(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_Y(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Z })]
    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_Z(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/CrossProduct)]
    #[inline(always)]
    pub fn cross(&self, other : Vector<3>) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_CrossProduct(self.to_opaque(), other.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/DirectionName)]
    #[inline(always)]
    pub fn name(&self) -> Direction { unsafe {
        Direction::from_string_unchecked(DF_ACTION__SetVariable_DirectionName(self.to_opaque()))
    } }

}


impl _VectorMethods<3> for Vector<3> {

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    #[inline(always)]
    fn new(lanes : [Float; 3]) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_Vector(lanes[0usize], lanes[1usize], lanes[2usize])
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    #[inline(always)]
    fn splat<F : Into<Float>>(lanes : F) -> Vector<3> { unsafe {
        let lanes = lanes.into();
        DF_ACTION__SetVariable_Vector(lanes, lanes, lanes)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    unsafe fn lane_unchecked<U : Into<UInt>>(&self, lane : U) -> Float { unsafe {
        let lanes = DF_ACTION__SetVariable_CreateList( String::from("X"), String::from("Y"), String::from("Z") );
        let lane  = DF_ACTION__SetVariable_Specialcharplus( lane.into(), UInt::from(1usize) );
        let lane  = DF_ACTION__SetVariable_GetListValue( lanes.to_opaque(), lane );
        DF_ACTION__SetVariable_GetVectorComp_Component_DynamicX(lane, self.to_opaque())
    } }


    #[doc(hidden)]
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self.inner._opaque_type) } }

    #[doc(hidden)]
    #[inline(always)]
    fn to_string(&self) -> String { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(&self.inner)
    } }

    #[doc(hidden)]
    #[inline(always)]
    fn to_text(&self) -> Text { unsafe {
        DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces(&self.inner)
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;
    fn DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces( ... ) -> Text;

    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_Vector( x : Float, y : Float, z : Float ) -> Vector<3>;

    fn DF_ACTION__SetVariable_SetVectorComp_Component_X( vector : DFOpaqueValue, x : Float ) -> Vector<3>;
    fn DF_ACTION__SetVariable_SetVectorComp_Component_Y( vector : DFOpaqueValue, y : Float ) -> Vector<3>;
    fn DF_ACTION__SetVariable_SetVectorComp_Component_Z( vector : DFOpaqueValue, z : Float ) -> Vector<3>;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_X( vector : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_Y( vector : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_Z( vector : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_DynamicX( component : String, vector : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_CrossProduct( a : DFOpaqueValue, b : DFOpaqueValue ) -> Vector<3>;
    fn DF_ACTION__SetVariable_DirectionName( vector : DFOpaqueValue ) -> String;

    fn DF_ACTION__SetVariable_CreateList( ... ) -> List<String>;
    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> String;

}
