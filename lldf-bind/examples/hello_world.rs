#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]


use lldf_bind::prelude::*;


#[event(PlayerJoin)]
fn player_join() {
    let default_player = Game::default_player();
    let all_players = Game::all_players();

    let player_uuid = default_player.uuids()[0usize];
    let player_name = default_player.names()[0usize];
    all_players.send_message("Hello,");
    all_players.send_message(player_uuid);
    all_players.send_message(player_name);
}
