use super::*;


impl Vector<3> {

    // TODO: with_x

    // TODO: with_y

    // TODO: with_z

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_X( self.clone() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Y })]
    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_Y( self.clone() )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp { Component = Z })]
    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_Z( self.clone() )
    } }

    // TODO: cross

    // TODO: name

}


impl _VectorMethods<3> for Vector<3> {

    #[lldf_bind_proc::dfdoc(SetVariable/Vector)]
    #[inline(always)]
    fn new(lanes : [Float; 3]) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_Vector(lanes[0usize], lanes[1usize], lanes[2usize])
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetVectorComp)]
    /// ##### Unsafe
    /// - **Does not do a bounds check**
    #[inline(always)]
    unsafe fn lane_unchecked<U : Into<UInt>>(&self, lane : U) -> Float { unsafe {
        let lanes = DF_ACTION__SetVariable_CreateList( String::from("X"), String::from("Y"), String::from("Z") );
        let lane  = DF_ACTION__SetVariable_Specialcharplus( lane.into(), UInt::from(1usize) );
        let lane  = DF_ACTION__SetVariable_GetListValue( lanes, lane );
        DF_ACTION__SetVariable_GetVectorComp_Component_DynamicX(lane, self.clone())
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( a : UInt, b : UInt ) -> UInt;

    fn DF_ACTION__SetVariable_Vector( x : Float, y : Float, z : Float ) -> Vector<3>;

    fn DF_ACTION__SetVariable_GetVectorComp_Component_X( vector : Vector<3> ) -> Float;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_Y( vector : Vector<3> ) -> Float;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_Z( vector : Vector<3> ) -> Float;
    fn DF_ACTION__SetVariable_GetVectorComp_Component_DynamicX( component : String, vector : Vector<3> ) -> Float;

    fn DF_ACTION__SetVariable_CreateList( ... ) -> List<String>;
    fn DF_ACTION__SetVariable_GetListValue( list : List<String>, index : UInt ) -> String;

}
