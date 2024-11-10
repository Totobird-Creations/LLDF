use super::*;


impl Vector<1> {

    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetListValue( self.clone(), UInt::from(1usize) )
    } }

}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_ACTION__SetVariable_GetListValue( list : Vector<1>, index : UInt ) -> Float;

}
