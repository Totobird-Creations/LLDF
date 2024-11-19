#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]

use lldf_bind::prelude::*;


mod items;

mod queue;
mod games;


#[event(PlotStart)]
fn plot_start() {
    items::setup();
    queue::setup();
    games::setup();
}

#[event(PlayerJoin)]
fn player_join(default : PlayerSel) {
    default.set_gamemode_adventure();
    default.set_item_in_slot(5usize, items::queue_disabled().clone());
}

#[event(PlayerRightClick)]
fn player_right_click(default : PlayerSel, item : Item) {
    if (item == items::queue_disabled()) {
        queue::add(&default);
    } else if (item == items::queue_enabled()) {
        queue::remove(&default);
    }
}

#[event(PlayerLeave)]
fn player_leave(default : PlayerSel) {
    queue::remove(&default);
    games::remove(&default);
}
