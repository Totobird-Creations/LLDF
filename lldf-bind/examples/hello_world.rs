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

    //default.send_message("Hello!".to_string());

    test(&default, Enum::A(String::from("Hello!")));
    test(&default, Enum::B(Item::diamond_axe()));

    let mut i = 0;
    while (i < 3) {
        default.send_message(String::from("BEEP"));
        i = i + 1;
    }

}


#[inline(never)]
fn test(sel : &PlayerSel, e : Enum) {
    match (e) {
        Enum::A(string) => sel.send_message(string),
        Enum::B(item) => sel.give_item(item),
    }
}
