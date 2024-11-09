#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    test(&default, NameColour::Aqua);
    default.send_message(String::from("Hello"));
    test(&default, NameColour::DarkGreen);
    //default.set_name_colour(NameColour::DarkGreen);
}

#[inline(never)]
fn test(s : &PlayerSel, c : NameColour) {
    s.set_name_colour(c);
}
