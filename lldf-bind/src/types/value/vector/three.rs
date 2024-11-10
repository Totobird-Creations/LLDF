use super::*;


impl Vector<3> {

    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_X( self.clone() )
    } }

    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_Y( self.clone() )
    } }

    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetVectorComp_Component_Z( self.clone() )
    } }

    // TODO: cross

    // TODO: name

}


impl _VectorMethods<3> for Vector<3> {

    #[inline(always)]
    fn new(lanes : [Float; 3]) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_Vector(lanes[0usize], lanes[1usize], lanes[2usize])
    } }

    #[inline(always)]
    fn lane<U : Into<UInt>>(&self, lane : U) -> Float { unsafe { // TODO: Add a bounds check.
        let lanes = DF_ACTION__SetVariable_CreateList( String::from("X"), String::from("Y"), String::from("Z") );
        let lane  = DF_ACTION__SetVariable_Specialcharplus( lane.into(), UInt::from(1usize) );
        let lane  = DF_ACTION__SetVariable_GetListValue( lanes, lane );
        DF_ACTION__SetVariable_GetVectorComp_Component_DynamicX(lane, self.clone())
    } }

}


#[allow(clashing_extern_declarations)]
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
