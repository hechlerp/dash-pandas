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
   pub players: Vec<player::PlayerCharacter>
}

impl ServerGameState {
    pub fn new(players: Vec<player::PlayerCharacter>) -> Self {
        let mut grid = createBlankGrid();

        //borders
        let borders = createBorders();
        for wall_tuple in borders {
            grid[wall_tuple.0][wall_tuple.1] = CELLVAL::Wall;
        }

        //walls

        let wallSpawns: Vec<(usize, usize)> = vec![(1,1), (2,1), (3,1), (15,1), (16,1),
        (9,2), (11,2),
        (3,3), (4,3), (6,3), (7,3), (8,3), (9,3), (11,3), (12,3), (15,3),
        (4,4), (11,4), (15,4),
        (3,5), (4,5), (9,5), (11,5),
        (1,6), (7,6), (9,6), (13,6), (14,6),
        (1,7), (5,7), (6,7), (7,7), (9,7), (10,7), (11,7)
        ];
        
        for wallTuple in wallSpawns {
            grid[wallTuple.0][wallTuple.1] = CELLVAL::Wall;
        }


        //players

        Self {
            grid,
            players
        }
    }

    pub fn get_grid(&self) -> Vec<Vec<CELLVAL>> {
        return self.grid.clone();
    }
}

fn init_server(players: Vec<String>) -> ServerGameState {
    os::server::write!(FP_GAME_INIT, true);
    let initial_state = ServerGameState::new(
        players
        .into_iter()
        .map(|player_id| player::PlayerCharacter::new(player_id))
        .collect()
    );
    os::server::write!(FP_GAME_STATE, initial_state);
    return initial_state;
}

fn join_lobby(player: String) -> usize {
    os::server::log!("joining lobby");
    let mut state = os::server::read_or!(ServerGameState, FP_GAME_STATE, init_server(vec![player.clone()]));
    let player_count = state.players.len();
    os::server::log!("{} players", state.players.len());
    let mut user = player::PlayerCharacter::new(player.clone());
    if player_count < MAX_PLAYERS {
        user.playerNum = player_count;
        state.players.push(user);
    } else {
        user.playerNum = 0;
        state.players = vec![user];
    }
    return os::server::COMMIT;
    
}


#[export_name = "turbo/join_server"]
unsafe extern "C" fn on_server_join() -> usize {
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
    return os::server::COMMIT;
}