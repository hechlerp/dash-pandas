use std::collections::btree_map::Keys;

use crate::*;
use crate::constants::{FP_GAME_STATE, FP_GAME_INIT};
use crate::env::{PROJECT_NAME};
use crate::gameserver::{join_server, ServerGameState};
use constants::CELL_SIZE;
use turbo::os::client::channel::*;

pub fn render() {
    // Subscribe to channel
    let multiplayer_dungeon_channel = Channel::subscribe(
        env::PROJECT_NAME,
        "dash-pandas-multiplayer-channel",
        &format!("{}", "bigboi"),
    );

    // Connect to channel
    if let Channel::Disconnected(ref conn) = multiplayer_dungeon_channel {
        conn.connect();
    };

    // Receive messages from the channel
    if let Channel::Connected(ref conn) = multiplayer_dungeon_channel {
        let t = tick();
        while let Ok(Some(data)) = conn.recv() {
            // Parse message
            if let Ok(msg) = String::from_utf8(data) {
                // Update player emote
                log!("{}", msg);
            }
        }
    }

    let server_inited = os::client::watch_file(PROJECT_NAME, FP_GAME_INIT)
        .data
        .and_then(|file| bool::try_from_slice(&file.contents).ok())
        .unwrap_or(false);

    let server_game_state = os::client::watch_file(PROJECT_NAME, FP_GAME_STATE)
        .data
        .and_then(|file| ServerGameState::try_from_slice(&file.contents).ok());

    if gamepad(0).start.pressed() {
        os::client::exec(PROJECT_NAME, "reset_game", &[]);
    }


    match (server_inited, server_game_state) {
        (server_inited, Some(server_game_state)) => {
            // logic
            
            let client_id = os::client::user_id().unwrap();
            let player_character = server_game_state.players.into_iter().find(|player| player.playerId == client_id);

            let grid = server_game_state.grid;
    

            if gamepad(0).a.just_pressed() {
                for y in 0..constants::MAP_DIM_Y {
                    log!("{:?}", grid[y]);
                }
            }

            log!("GRID");
            log!("{:?}", grid);
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    match grid[y][x] {
                        CELLVAL::Empty => {},
                        CELLVAL::Wall => {
                            sprite!(
                                "dumpster-top", x = x * CELL_SIZE, y = y * CELL_SIZE
                            );
                        },
                        CELLVAL::P1 => {
                            log!("P1 Pos: {}, {}", x, y);
                            sprite!(
                                "Racoon_Main_UpDash_shadow", x = x * CELL_SIZE, y = y * CELL_SIZE
                            );
                        },
                        CELLVAL::P2 => {
                            sprite!(
                                "Racoon_Main_UpDash_shadow", x = x * CELL_SIZE, y = y * CELL_SIZE
                            );
                        },
                        CELLVAL::NotAssigned => {}
                    }
                }
            }
            
            // for y in 0..constants::MAP_DIM_Y {
            //     for x in 0..constants::MAP_DIM_X {
            //         match grid[y][x] {
            //             CELLVAL::Empty => {},
            //             CELLVAL::Wall => {
            //                 sprite!(
            //                     "dumpster-top", x = x * CELL_SIZE, y = y * CELL_SIZE
            //                 );
            //             },
            //             CELLVAL::P1 => {
            //                 sprite!(
            //                     "Racoon_Main_UpDash_shadow", x = x * CELL_SIZE, y = y * CELL_SIZE
            //                 );
            //             },
            //             CELLVAL::P2 => {
            //                 sprite!(
            //                     "Racoon_Main_UpDash_shadow", x = x * CELL_SIZE, y = y * CELL_SIZE
            //                 );
            //             },
            //             CELLVAL::NotAssigned => {}
            //         }
        
            //         // log!("Nested loop: i = {}, j = {}", i, j);
            //     }
            // }
            if player_character != None {
                let mut confirmed_character = player_character.unwrap();
                
                let mut isAttemptingMove: bool = false;
                let mut dir = DIRECTIONS::Left;
                if gamepad(0).left.just_pressed() {
                    isAttemptingMove = true;
                    dir = DIRECTIONS::Left;
                }
                if gamepad(0).right.just_pressed() {
                    isAttemptingMove = true;
                    dir = DIRECTIONS::Right;
                }
                if gamepad(0).up.just_pressed() {
                    isAttemptingMove = true;
                    dir = DIRECTIONS::Up;
                }
                if gamepad(0).down.just_pressed() {
                    isAttemptingMove = true;
                    dir = DIRECTIONS::Down;
                }
                if isAttemptingMove {
                    let args = borsh::to_vec(&(client_id, dir)).unwrap();
                    os::client::exec(env::PROJECT_NAME, "attempt_move", &args);
                }
                // sprite!(
                //     "Racoon_Main_UpDash_shadow", x = confirmed_character.position.0 * CELL_SIZE, y = confirmed_character.position.1 * CELL_SIZE
                // );
                // if gamepad(0).start.just_pressed() {
                //     if let Some(player_character) = player_character {
                //         // drawCharacter(playerCharacter.position.x, playerCharacter.position.y)
                //         //log!("{:?}", player_character.position);
                //     }
                // }
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