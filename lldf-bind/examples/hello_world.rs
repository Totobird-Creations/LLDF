#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join(default : PlayerSel) {
    let loc = Location::new(1.0, 2.0, 3.0, Float::from(68.234).to_radians(), 5.0);
    default.send_message(loc);
}
