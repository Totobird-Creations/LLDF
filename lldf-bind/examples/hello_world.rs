#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerSwapHands)]
fn player_swap_hands(default : PlayerSel) {
    default.give_potion(Potion::dolphins_grace().with_amplifier(2usize).with_duration(100usize), Flag::True, Particles::Ambient)
}

