#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


mod queue;
mod items;

use lldf_bind::prelude::*;


#[event(PlotStart)]
fn plot_start() {
    queue::setup();
    items::setup();
}


#[event(PlayerJoin)]
fn player_join(default : PlayerSel) {
    default.set_gamemode_adventure();
    default.set_item_in_slot(5usize, items::queue_disabled());
}
