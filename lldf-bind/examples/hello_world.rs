#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join() {
    let default_player = Game::default_player();
    let all_players = Game::all_players();

    let mut player_name = default_player.names().clone();

    all_players.send_message(player_name[0usize]);

    player_name[0usize] = player_name[1usize];

    all_players.send_message(player_name[0usize]);
}
