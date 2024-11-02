#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


struct Potato {
    a : &'static str,
    b : &'static str
}


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    let mut p = Potato { a : "Hello,", b : "World!" };
    default.send_message(p.a);
    default.send_message(p.b);
    p.a = "LLDF!";
    default.send_message(p.a);
    default.send_message(p.b);
}
