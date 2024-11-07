#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


struct Potato {
    a : Text,
    b : Text
}

#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    let mut p = Potato {
        a : Text::from("Hello,"),
        b : Text::from("World!")
    };
    print_potato(&default, &p);
    do_something(&mut p.b);
    print_potato(&default, &p);
}

#[inline(never)]
#[no_mangle]
fn do_something(swapping_ptr : &mut Text) {
    *swapping_ptr = Text::from("LLDF!");
}

#[inline(never)]
#[no_mangle]
fn print_potato(player : &PlayerSel, potato : &Potato) {
    player.send_message(&potato.a);
    player.send_message(&potato.b);
}

