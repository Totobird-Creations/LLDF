use super::*;


use lldf_bind::prelude::*;
use core::mem::MaybeUninit;


static mut QUEUED_UUIDS : MaybeUninit<List<Uuid>> = MaybeUninit::uninit();


pub fn setup() {
    *unsafe{ QUEUED_UUIDS.assume_init_mut() } = List::new();
    Thread::spawn(main_loop);
}

pub fn add(player : &PlayerSel) {
    unsafe{ QUEUED_UUIDS.assume_init_mut() }.append(player.uuids().clone());
    player.set_item_in_slot(5usize, items::queue_enabled().clone());
}

pub fn remove(player : &PlayerSel) {
    unsafe{ QUEUED_UUIDS.assume_init_mut() }.erase(unsafe{ &player.uuid_unchecked() });
    player.set_item_in_slot(5usize, items::queue_disabled().clone());
}

pub fn count() -> UInt {
    unsafe{ QUEUED_UUIDS.assume_init_ref() }.len()
}

fn pop_next_round() -> Option<List<Uuid>> {
    if (queue::count() >= 2usize) {
        let     queued_uuids = unsafe{ QUEUED_UUIDS.assume_init_mut() };
        let mut round_uuids  = List::new();
        unsafe{
            round_uuids.push(queued_uuids.remove_unchecked(0usize));
            round_uuids.push(queued_uuids.remove_unchecked(0usize));
        }
        Some(round_uuids)
    } else {
        None
    }
}


fn main_loop() {
    loop {
        Thread::sleep(10usize);

        Game::all_players().send_message(unsafe{ QUEUED_UUIDS.assume_init_ref() }.clone());

        if let Some(round_uuids) = pop_next_round() {
            let round_players = unsafe{ PlayerSel::from_uuids_unchecked(round_uuids) };
            round_players.send_title("Starting game.".to_string(), "".to_string(), 0usize, 50usize, 10usize);
            round_players.clear_inventory();
        }

    }
}
