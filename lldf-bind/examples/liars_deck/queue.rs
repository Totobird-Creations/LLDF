use super::*;


use lldf_bind::prelude::*;
use core::mem::MaybeUninit;


static mut QUEUED_UUIDS : MaybeUninit<List<Uuid>> = MaybeUninit::uninit();


#[inline(always)]
pub fn setup() {
    *unsafe{ QUEUED_UUIDS.assume_init_mut() } = List::new();
    Thread::spawn(main_loop);
}

#[inline(always)]
pub fn add(player : &PlayerSel) {
    unsafe{ QUEUED_UUIDS.assume_init_mut() }.append(player.uuids().clone());
    player.give_item(items::queue_enabled());
}

#[inline(always)]
pub fn remove(player : &PlayerSel) {
    unsafe{ QUEUED_UUIDS.assume_init_mut() }.erase(unsafe{ &player.uuid_unchecked() });
    player.give_item(items::queue_disabled());
}

#[inline(always)]
pub fn count() -> UInt {
    unsafe{ QUEUED_UUIDS.assume_init_ref() }.len()
}


fn main_loop() {
    loop {
        Thread::sleep(10usize);

        Game::all_players().send_message(unsafe{ QUEUED_UUIDS.assume_init_ref() }.clone());
    }
}
