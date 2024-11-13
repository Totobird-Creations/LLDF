#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


pub enum Enum {
    A(String),
    B(Item)
}


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    test(&default, Enum::A(String::from("Hello!")));
    test(&default, Enum::B(Item::diamond_axe()));
}


#[inline(never)]
fn test(sel : &PlayerSel, e : Enum) {
    match (e) {
        Enum::A(string) => sel.send_message(string),
        Enum::B(item) => sel.give_item(item),
    }
}
