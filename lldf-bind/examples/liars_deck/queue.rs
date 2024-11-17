use lldf_bind::prelude::*;
use core::mem::MaybeUninit;


static mut QUEUED_UUIDS : MaybeUninit<List<Uuid>> = MaybeUninit::uninit();


pub fn setup() {
    *unsafe{ QUEUED_UUIDS.assume_init_mut() } = List::new();
    Thread::spawn(main_loop);
}

pub fn add(uuid : Uuid) {
    unsafe{ &mut*QUEUED_UUIDS.as_mut_ptr() }.push(uuid);
}

pub fn remove(uuid : &Uuid) {
    unsafe{ QUEUED_UUIDS.assume_init_mut() }.erase(uuid)
}

pub fn count() -> UInt {
    unsafe{ QUEUED_UUIDS.assume_init_ref() }.len()
}


fn main_loop() {
    loop {
        Thread::sleep(10usize);

        
    }
}
