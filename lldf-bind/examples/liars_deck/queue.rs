use lldf_bind::prelude::*;
use core::mem::MaybeUninit;


static mut QUEUED_UUIDS : MaybeUninit<List<Uuid>> = MaybeUninit::uninit();


pub fn setup() {
    *unsafe{ QUEUED_UUIDS.assume_init_mut() } = List::new();
    //Game::spawn(main_loop)
}

pub fn add(uuid : Uuid) {
    unsafe{ &mut*QUEUED_UUIDS.as_mut_ptr() }.push(uuid);
    //unsafe{ PlayerSel::from_uuid_unchecked(uuid) }.send_message("Joined queue".to_string());
}

pub fn remove(uuid : Uuid) {
    unsafe{ QUEUED_UUIDS.assume_init_mut() }.erase(&uuid)
}


fn main_loop() {
    loop {
        Game::sleep(20usize);
        Game::all_players().send_message("Test".to_string());
    }
}
