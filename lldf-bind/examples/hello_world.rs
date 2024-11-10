#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {

    let mat = Matrix::translation(1.0, 2.0, 3.0);
    default.send_message(mat.transpose().into_inner());

}

