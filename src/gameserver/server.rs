use crate::env::{PROJECT_NAME};
use crate::*;
use crate::constants::{MAX_PLAYERS, DIRECTIONS, CELLVAL, MAP_DIM_X, MAP_DIM_Y, FP_GAME_STATE, FP_GAME_INIT};
use crate::gameserver::player;

pub fn join_server() {

    os::client::exec(PROJECT_NAME, "join_server", &[]);
    
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ServerGameState {
   pub grid: Vec<Vec<CELLVAL>>,
   pub players: Vec<player::PlayerCharacter>,
   pub is_winner: bool,
   pub winning_player_num: usize
}

impl ServerGameState {
    pub fn new() -> Self {
        let mut grid = createBlankGrid();

        //borders
        let borders = createBorders();
        for wall_tuple in borders {
            grid[wall_tuple.1][wall_tuple.0] = CELLVAL::Wall;
        }

        //walls

        let wallSpawns: Vec<(usize, usize)> = vec![/* (1,1), (2,1), (3,1), (15,1), (16,1),
        (9,2), (11,2),
        (3,3), (4,3), (6,3), (7,3), (8,3), (9,3), (11,3), (12,3), (15,3),
        (4,4), (11,4), (15,4),
        (3,5), (4,5), (9,5), (11,5),
        (1,6), (7,6), (9,6), (13,6), (14,6),
        (1,7), (5,7), (6,7), (7,7), (9,7), (10,7), (11,7) */
        ];
        
        for wallTuple in wallSpawns {
            grid[wallTuple.1][wallTuple.0] = CELLVAL::Wall;
        }


        //players

        Self {
            grid,
            players: vec![],
            is_winner: false,
            winning_player_num: 20
        }
    }

    pub fn get_grid(&self) -> Vec<Vec<CELLVAL>> {
        return self.grid.clone();
    }

    pub fn updateGrid(&mut self, updates: Vec<((usize, usize), CELLVAL)>) {
        for update in updates {
            let (grid_pos, next_val) = update;
            let (grid_x, grid_y) = grid_pos;
            self.grid[grid_y][grid_x] = next_val;
        }
    }

    pub fn win_game(&mut self, winner: &PlayerCharacter) {
        self.is_winner = true;
        self.winning_player_num = winner.playerNum
    }
}

fn init_server() -> ServerGameState {
    os::server::log!("initting server");
    os::server::write!(FP_GAME_INIT, true);
    let mut initial_state: ServerGameState = ServerGameState::new();
    os::server::write!(FP_GAME_STATE, &initial_state);
    return initial_state;
}

fn join_lobby(player: String) -> usize {
    os::server::log!("joining lobby");
    let mut state = os::server::read_or!(ServerGameState, FP_GAME_STATE, init_server());
    let player_count = state.players.len();
    os::server::log!("{} players", state.players.len());
    let mut user = player::PlayerCharacter::new(player.clone(), if player_count < MAX_PLAYERS {player_count } else { 0 });
    os::server::log!("Joined Player Pos: ({}, {})", user.position.0, user.position.1);

    let playerPos: &(usize, usize) = &user.position;
    state.grid[playerPos.1][playerPos.0] = user.assingedCellVal.clone();

    if player_count < MAX_PLAYERS {
        state.players.push(user);
        os::server::log!("Player joined success");
    } else {
        state.players = vec![];
        state.players.push(user);
    }
    state.is_winner = false;
    state.winning_player_num = 20;
    os::server::write!(FP_GAME_STATE, state);
    return os::server::COMMIT;
    
}

fn leave_lobby(player: String) -> usize {
    let mut state = os::server::read!(ServerGameState, FP_GAME_STATE);
    state.players.retain(|statefulPlayer| statefulPlayer.playerId != player);
    os::server::write!(FP_GAME_STATE, state);
    return os::server::COMMIT;
}

fn get_player_by_id(state: &ServerGameState, id: String) -> Option<PlayerCharacter> {
    return state.players.clone().into_iter().find(|player| player.playerId == id);
}

fn get_grid(state: &ServerGameState) -> Vec<Vec<CELLVAL>> {
    return state.get_grid();
}

fn get_state() -> ServerGameState {
    return os::server::read_or!(ServerGameState, FP_GAME_STATE, init_server());
}


#[export_name = "turbo/join_server"]
unsafe extern "C" fn on_server_join() -> usize {
    let userData = os::server::get_command_data();
    let userId: String = String::from_utf8(userData).unwrap();
    os::server::log!("user joined!");
    let has_been_initialized: bool = os::server::read_or!(bool, FP_GAME_INIT, false);
    if !has_been_initialized {
        init_server();
    }
    // return os::server::COMMIT;
    return join_lobby(userId);
}

#[export_name = "turbo/leave_server"]
unsafe extern "C" fn on_server_leave() -> usize {
    let user: String = os::server::get_user_id();
    return leave_lobby(user);
}

#[export_name = "channel/dash-pandas-multiplayer-channel"]
unsafe extern "C" fn on_connect() {
    use turbo::os::server::*;
    loop {
        match os::server::channel_recv() {
            Ok(ChannelMessage::Connect(user_id, data)) => {
                // let emote = Emote::try_from_slice(&data).unwrap();
                // let payload = (user_id, emote);
                // join_lobby(user_id);
                os::server::enqueue_command (
                    PROJECT_NAME, 
                    "join_server", 
                    user_id.as_bytes(),
                    os::server::random_number(), 
                    None
                );
                os::server::channel_broadcast(b"asdf");
            }
            Err(_err) => return,
            _ => {}
        }
    }
}

#[export_name = "turbo/attempt_move"]
unsafe extern "C" fn on_attempt_move() -> usize {
    os::server::log!("attempting move...");
    let ( user_id, dir ) = os::server::command!((String, DIRECTIONS));
    let mut state = get_state();
    let found_player: Option<PlayerCharacter> = get_player_by_id(&state, user_id);
    if (found_player == None) {
        return os::server::CANCEL;
    }
    let mut character = found_player.unwrap();
    let (nextPos, didEncounterFoe) = /* ((1,1), false); */ character.getMovementSpaceInDir(dir, &get_grid(&state));
    let prev_pos = character.position.clone();
    character.position = nextPos;
    if didEncounterFoe {
        state.win_game(&character);
    }
    os::server::log!("{:?}", nextPos);
    if nextPos.0 != prev_pos.0 || nextPos.1 != prev_pos.1 {
        state.updateGrid(vec![(prev_pos, CELLVAL::Empty), (nextPos, character.assingedCellVal)]);
    }
    os::server::write!(FP_GAME_STATE, state);
    return os::server::COMMIT;
}

#[export_name = "turbo/auto_win"]
unsafe extern "C" fn on_auto_win() -> usize {
    
    let user_id = os::server::command!(String);
    let mut state = get_state();
    let found_player: Option<PlayerCharacter> = get_player_by_id(&state, user_id);
    if (found_player == None) {
        return os::server::CANCEL;
    }
    let mut character = found_player.unwrap();
    state.win_game(&character);
    os::server::write!(FP_GAME_STATE, state);
    return os::server::COMMIT;
}

#[export_name = "turbo/reset_game"]
unsafe extern "C" fn on_reset() -> usize {
    let mut old_state = get_state();

    let mut state = ServerGameState::new();
    for player in old_state.players {
        state.players.push(PlayerCharacter::new(player.playerId, player.playerNum));
    }
    os::server::write!(FP_GAME_STATE, state);


    return os::server::COMMIT;
}