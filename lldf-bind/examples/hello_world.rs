#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


struct Potato {
    a : String,
    b : String
}


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    let mut p = Potato {
        a : String::from("Hello,"),
        b : String::from("World!")
    };
    default.send_message(Text::from(&p.a));
    default.send_message(Text::from(&p.b));
    do_something(&mut p.b);
    default.send_message(Text::from(&p.a));
    default.send_message(Text::from(&p.b));
}


#[inline(never)]
#[no_mangle]
fn do_something(swapping_ptr : &mut String) {
    *swapping_ptr = String::from("LLDF!");
}


//#[inline(never)]
//fn print_potato(player : &PlayerSel, potato : &Potato) {
//    player.send_message(Text::from(&potato.a));
//    player.send_message(Text::from(&potato.b));
//}
