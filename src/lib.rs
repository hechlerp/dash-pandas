mod environment;
use std::collections::HashMap;
use constants::{CELLVAL, DIRECTIONS};
use environment::{createBlankGrid, createBorders};
mod constants;
mod env;
mod client;
mod gameserver;

turbo::cfg! {r#"
    name = "dash-pandas"
    version = "1.0.0"
    author = "Turbo"
    description = "Your first turbo os program"
    [settings]
    resolution = [576, 288]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
    struct GameState {
        pub frameNum: u32,
        pub grid: Vec<Vec<CELLVAL>>
    } = {
        Self::new()
    }
}

impl GameState {
    fn new() -> Self {
        let mut grid: Vec<Vec<CELLVAL>> = createBlankGrid();
        // let borders: Vec<(usize, usize)> = createBorders();

        // for wallTuple in borders {
        //     grid[wallTuple.0][wallTuple.1] = CELLVAL::Wall;
        //     sprite!(
        //         "Racoon_Main_UpDash_shadow", x = wallTuple.0 * 32, y = -(wallTuple.1 as isize) * 32
        //     );
        // }
        // let wallSpawns: Vec<(usize, usize)> = vec![
        //     (1,1), (2,1), (3,1), (15,1), (16,1), (9,2), (11,2),
        //     (3,3), (4,3), (6,3), (7,3), (8,3), (9,3), (11,3), (12,3), (15,3),
        //     (4,4), (11,4), (15,4), (3,5), (4,5), (9,5), (11,5),
        //     (1,6), (7,6), (9,6), (13,6), (14,6),
        //     (1,7), (5,7), (6,7), (7,7), (9,7), (10,7), (11,7)
        // ];
        // for wallTuple in wallSpawns {
        //     grid[wallTuple.0][wallTuple.1] = CELLVAL::Wall;
        // }


        Self {
            grid,
            frameNum: 0
        }
    }
}

turbo::go!({
    let mut state = GameState::load();
    clear!(0xADD8E6FF);
    let (x, y, w, h) = (36, 102, 60, 20);
    let mut color = 0x00008BFF;

    state.frameNum += 1;

    // log!("DEBUG: {:?}", state.frameNum);

    // // debug all grid valuse with frame number
    // for y in 0..constants::MAP_DIM_Y {

    //     log!("NEXT ROW");

    //     for x in 0..constants::MAP_DIM_X {
    //         log!("DEBUG: frameNum: {}, grid[{}][{}]: {:?}", state.frameNum, x, y, state.grid[x][y]);
    //     }
    // }

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

    client::render();

    //sprite!(
    //    "Racoon_Main_UpDash_shadow"
    //);
    for j in 0..constants::MAP_DIM_Y {
        for i in 0..constants::MAP_DIM_X {
            match state.grid[i][j] {
                CELLVAL::Empty => {},
                CELLVAL::Wall => {
                    sprite!(
                        "Racoon_Main_UpDash_shadow", x = i * 32, y = (j as isize) * 32
                    );
                },
                CELLVAL::P1 => {
                    sprite!(
                        "Racoon_Main_UpDash_shadow", x = i * 32, y = (j as isize) * 32
                    );
                },
                CELLVAL::P2 => {
                    sprite!(
                        "Racoon_Main_UpDash_shadow", x = i * 32, y = (j as isize) * 32
                    );
                }
            }

            // log!("Nested loop: i = {}, j = {}", i, j);
        }
    }
    state.save();
});

// #[export_name = "turbo/input_left"]
// unsafe extern "C" fn on_input_left() -> usize {
//     os::server::log!("input_left");
//     return os::server::COMMIT;
// }
// #[export_name = "turbo/input_right"]
// unsafe extern "C" fn on_input_right() -> usize {
//     os::server::log!("input_right");
//     return os::server::COMMIT;
// }

// #[export_name = "turbo/input_up"]
// unsafe extern "C" fn on_input_up() -> usize {
//     os::server::log!("input_up");
//     return os::server::COMMIT;
// }

// #[export_name = "turbo/input_down"]
// unsafe extern "C" fn on_input_down() -> usize {
//     os::server::log!("input_down");
//     return os::server::COMMIT;
// }