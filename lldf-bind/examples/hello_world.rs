#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


pub enum Enum {
    A(String),
    B(Item)
}


#[event(PlayerJoin)]
fn player_swap_hands(default : PlayerSel) {

    if (test(&default, true, true)) {
        default.send_message("YES!".to_string());
    } else {
        default.send_message("no.".to_string());
    }

}


#[inline(never)]
#[no_mangle]
fn test(sel : &PlayerSel, a : bool, b : bool) -> bool {
    a && b
}
