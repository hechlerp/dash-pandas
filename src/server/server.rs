use crate::env;
use crate::turbo::*;
use crate::constants::{MAX_PLAYERS, DIRECTIONS};

pub fn joinServer() -> bool {

    let response: usize = os::client::exec(PROJECT_NAME, "join_server", ());
    if (response == os::server::COMMIT) {
        return true;
    }
    
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
struct ServerGameState {
    grid: Vec<Vec<bool>>,
    players: Vec<String>
}

impl ServerGameState {
    fn new(players: Vec<String>) -> Self {
        let mut grid = createBlankGrid();
        for wallTuple in borders {
            grid[wallTuple.0][wallTuple.1] = true;
        }
        Self {
            grid,
            players
        }
    }

    fn getGrid() -> Vec<Vec<bool>> {
        return grid;
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

fn initServer(players: Vec<String>) -> ServerGameState {
    os::server::write!("game_init", true);
    let initialState = ServerGameState::new(players);
    os::server::write!(ServerGameState, "game_state");
    return initialState;
}

fn join_lobby(player: String) -> usize {
    let state = os::server::read!(ServerGameState, "game_state");
    if state.players.len() <= MAX_PLAYERS {
        state.players.push(user);
        return os::server::COMMIT;
    } else {
        return os::server::CANCEL;
    }
}


#[export_name = "turbo/join_server"]
unsafe extern "C" fn on_server_join() -> usize {
    let user: &str = os::server::get_user_id();
    os::server::log!(format!("player joined-{}", user));
    let hasBeenInitialized = os::server::read_or!(bool, "game_init", false);
    if (!hasBeenInitialized) {
        let serverState = initServer();
    }
    return join_lobby(user);
}