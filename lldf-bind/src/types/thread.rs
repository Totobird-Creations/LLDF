use crate::prelude::*;


#[repr(transparent)]
pub struct Thread {
    inner : u64
}


/// `START_PROCESS`
impl Thread {

    #[inline(always)]
    pub fn spawn(process : fn() -> ()) -> Thread { unsafe {
        DF_PROCESS__Spawn(process)
    } }

    #[inline(always)]
    pub fn join(self) -> () { unsafe {
        DF_PROCESS__Join(self)
    } }

}

/// `CONTROL`
impl Thread {

    #[lldf_bind_proc::dfdoc(Control/Wait { TimeUnit = Ticks })]
    #[inline(always)]
    pub fn sleep<U : Into<UInt>>(time_ticks : U) -> () { unsafe {
        DF_ACTION__Control_Wait_TimeUnit_Ticks(time_ticks.into())
    } }

}



extern "C" {

    fn DF_PROCESS__Spawn( process : fn() -> () ) -> Thread;
    fn DF_PROCESS__Join( thread : Thread ) -> ();

    fn DF_ACTION__Control_Wait_TimeUnit_Ticks( time_ticks : UInt ) -> ();

}