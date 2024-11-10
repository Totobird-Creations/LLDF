use super::*;


impl Vector<4> {

    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(1usize) )
    } }

    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(2usize) )
    } }

    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(3usize) )
    } }

    #[inline(always)]
    pub fn w(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(4usize) )
    } }

}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<4>, index : UInt ) -> Float;

}
