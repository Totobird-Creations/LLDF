#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


//#[event(SwapHands)]
//fn swap_hands(default : PlayerSel) {
//    let number = Text::from(UInt::from(10usize));
//    let mut i = 0;
//    while (i < 50) {
//        default.send_message(&number);
//        i = i + 1;
//    }
//}


#[event(SwapHands)]
fn swap_hands(default : PlayerSel) {
    default.send_message("Welcome to the plot!");
}
