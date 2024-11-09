#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    let c = Colour::from_rgb(0usize, 127usize, 255usize);
    let c = c.hsb();
    default.send_message(c.0);
    default.send_message(c.1);
    default.send_message(c.2);
}

