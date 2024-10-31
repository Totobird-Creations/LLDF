#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


//#[event(Join)]
//fn player_join(default : PlayerSel) {
//    let number = Text::from(UInt::from(10usize));
//    let mut i = 0;
//    while (i < 50) {
//        default.send_message(&number);
//        i = i + 1;
//    }
//}


#[event(Join)]
fn player_join(default : PlayerSel) {
    default.send_message("Welcome to the plot!");
}
