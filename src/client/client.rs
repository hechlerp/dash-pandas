use crate::turbo::*;
use crate::constants::{FP_GAME_STATE, FP_GAME_INIT};
use crate::env;
use crate::server::{PlayerCharacter, join_server};

fn render() {
    let serverInited = os::client::watch_file(PROJECT_NAME, FP_GAME_INIT)
        .data
        .and_then(|file| data as bool)
        .unwrap_or(false);

    let serverGameState = os::client::watch_file(PROJECT_NAME, FP_GAME_STATE)
        .data
        .and_then(|file| ServerGameState::BorshDeserialize(file).ok())
        .unwrap_or(None);
    
    match (serverInited, serverGameState) {
        (Some(serverInited), Some(serverGameState)) => {
            // logic
            
            let clientId = os::client::get_user_id();
            let playerCharacter = serverGameState.players.find<PlayerCharacter>(|player| player.playerId == clientId);
            if playerCharacter != None {
                // drawCharacter(playerCharacter.position.x, playerCharacter.position.y)
                log(playerCharacter.position.0);
                log(playerCharacter.position.1);
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