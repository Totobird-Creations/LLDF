use super::*;


impl Vector<2> {

    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(1usize) )
    } }

    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(2usize) )
    } }

}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<2>, index : UInt ) -> Float;

}
