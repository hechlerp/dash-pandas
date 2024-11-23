use crate::env::{PROJECT_NAME};
use crate::*;
use crate::constants::{MAX_PLAYERS, MAP_DIM_X, MAP_DIM_Y, FP_GAME_STATE, FP_GAME_INIT};

pub fn join_server() {

    os::client::exec(PROJECT_NAME, "join_server", &[]);
    
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
struct ServerGameState {
    grid: Vec<Vec<CELLVAL>>,
    players: Vec<PlayerCharacter>
}

impl ServerGameState {
    fn new(players: Vec<PlayerCharacter>) -> Self {
        let mut grid = createBlankGrid();
        let borders = createBorders();
        for wall_tuple in borders {
            grid[wall_tuple.0][wall_tuple.1] = CELLVAL::Wall;
        }
        Self {
            grid,
            players
        }
    }

    fn get_grid(&self) -> Vec<Vec<CELLVAL>> {
        return self.grid.clone();
    }
}


#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
struct PlayerCharacter {
    position: (usize, usize),
    playerId: String
}

impl PlayerCharacter {
    fn new(player: String) -> Self {
        Self {
            position: (MAP_DIM_X / 2, MAP_DIM_Y / 2),
            playerId: player
        }
    }

    // fn move(nextX: usize, nextY: usize) {
    //     position = (nextX, nextY);
    // }
}

fn init_server(players: Vec<String>) -> ServerGameState {
    os::server::write!(FP_GAME_INIT, true);
    let initial_state = ServerGameState::new(
        players
        .into_iter()
        .map(|player_id| PlayerCharacter::new(player_id))
        .collect()
    );
    os::server::write!(FP_GAME_STATE, initial_state);
    return initial_state;
}

fn join_lobby(player: String) -> usize {
    let mut state = os::server::read!(ServerGameState, FP_GAME_STATE);
    if state.players.len() <= MAX_PLAYERS {
        let user = PlayerCharacter::new(player.clone());
        state.players.push(user);
        return os::server::COMMIT;
    } else {
        return os::server::CANCEL;
    }
    
}


#[export_name = "turbo/join_server"]
unsafe extern "C" fn on_server_join() -> usize {
    let user: String = os::server::get_user_id();
    os::server::log!("user joined!");
    let has_been_initialized = os::server::read_or!(bool, FP_GAME_INIT, false);
    if (!has_been_initialized) {
        init_server(vec![user.clone()]);
    }
    return join_lobby(user);
}