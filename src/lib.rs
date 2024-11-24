mod environment;
use std::collections::HashMap;
use constants::CELLVAL;
use environment::{createBlankGrid, createBorders};
use server::PlayerCharacter;
mod constants;
mod env;
mod server;

turbo::cfg! {r#"
    name = "dash-pandas"
    version = "1.0.0"
    author = "Turbo"
    description = "Your first turbo os program"
    [settings]
    resolution = [132, 224]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
    struct GameState {
        grid: Vec<Vec<CELLVAL>>,
        P1Char: PlayerCharacter,
    } = {
        Self::new()
    }
}

impl GameState {
    fn new() -> Self {
        let borders: Vec<(usize, usize)> = createBorders();
        let mut grid: Vec<Vec<CELLVAL>> = createBlankGrid();
        let wallSpawns: Vec<(usize, usize)> = vec![(3, 1)];

        for wallTuple in borders {
            grid[wallTuple.0][wallTuple.1] = CELLVAL::Wall;
        }
        for wallTuple in wallSpawns {
            grid[wallTuple.0][wallTuple.1] = CELLVAL::Wall;
        }
        
        Self {
            grid,
            P1Char: PlayerCharacter::new("player1".to_string())
        }
    }
}

turbo::go!({
    let mut state = GameState::load();

    clear!(0xADD8E6FF);
    let (x, y, w, h) = (36, 102, 60, 20);
    let mut color = 0x00008BFF;

    let m = mouse(0);
    //check if mouse is over the button and clicked
    if (m.position[0] >= x && m.position[0] <= x + w)
        && (m.position[1] >= y && m.position[1] <= y + h)
    {
        color = 0x4169E1FF;
        if m.left.just_pressed() {
            os::client::exec(env::PROJECT_NAME, "hello", &[]);
        }
    }
    //draw a button
    rect!(x = x, y = y, w = w, h = h, color = color, border_radius = 8);
    text!("HELLO!!", x = 50, y = 109);

    if gamepad(0).left.just_released() {
        os::client::exec(env::PROJECT_NAME, "input_left", &[]);
        state.P1Char.moveInDirection(constants::DIRECTIONS::Left, &state.grid);
        log!("DEBUG: {:?}", state.P1Char.position);
    }
    if gamepad(0).right.just_released() {
        os::client::exec(env::PROJECT_NAME, "input_right", &[]);
        state.P1Char.moveInDirection(constants::DIRECTIONS::Right, &state.grid);
        log!("DEBUG: {:?}", state.P1Char.position);
    }
    if gamepad(0).up.just_released() {
        os::client::exec(env::PROJECT_NAME, "input_up", &[]);
        state.P1Char.moveInDirection(constants::DIRECTIONS::Up, &state.grid);
        log!("DEBUG: {:?}", state.P1Char.position);
    }
    if gamepad(0).down.just_released() {
        os::client::exec(env::PROJECT_NAME, "input_down", &[]);
        state.P1Char.moveInDirection(constants::DIRECTIONS::Down, &state.grid);
        log!("DEBUG: {:?}", state.P1Char.position);
    }

    

    state.save();
});

#[export_name = "turbo/input_left"]
unsafe extern "C" fn on_input_left() -> usize {
    os::server::log!("input_left");
    return os::server::COMMIT;
}
#[export_name = "turbo/input_right"]
unsafe extern "C" fn on_input_right() -> usize {
    os::server::log!("input_right");
    return os::server::COMMIT;
}

#[export_name = "turbo/input_up"]
unsafe extern "C" fn on_input_up() -> usize {
    os::server::log!("input_up");
    return os::server::COMMIT;
}

#[export_name = "turbo/input_down"]
unsafe extern "C" fn on_input_down() -> usize {
    os::server::log!("input_down");
    return os::server::COMMIT;
}