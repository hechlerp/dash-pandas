use crate::*;
use crate::constants::{FP_GAME_STATE, FP_GAME_INIT};
use crate::env::{PROJECT_NAME};
use crate::gameserver::{join_server, ServerGameState};

pub fn render() {
    let server_inited = os::client::watch_file(PROJECT_NAME, FP_GAME_INIT)
        .data
        .and_then(|file| bool::try_from_slice(&file.contents).ok())
        .unwrap_or(false);

    let server_game_state = os::client::watch_file(PROJECT_NAME, FP_GAME_STATE)
        .data
        .and_then(|file| ServerGameState::try_from_slice(&file.contents).ok());
    
    match (server_inited, server_game_state) {
        (server_inited, Some(server_game_state)) => {
            // logic
            
            let client_id = os::client::user_id().unwrap();
            let player_character = server_game_state.players.iter().find(|player| player.playerId == client_id);
            if gamepad(0).start.just_pressed() {
                if let Some(player_character) = player_character {
                    // drawCharacter(playerCharacter.position.x, playerCharacter.position.y)
                    log!("{:?}", player_character.position);
                }
            }
        }
        _ => {
            if gamepad(0).start.just_pressed() {
                join_server();
            } 
            text!("press space to join!");
        }
    }
}