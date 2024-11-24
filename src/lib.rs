mod environment;
use std::collections::HashMap;
use constants::{CELLVAL, CELL_SIZE, DIRECTIONS};
use environment::{createBlankGrid, createBorders};
use gameserver::PlayerCharacter;
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
        // pub grid: Vec<Vec<CELLVAL>>,
        pub P1Char: PlayerCharacter,
        pub screenshake_timer: f32,
        pub move_delay_timer: f32,
        pub smoke_timer: f32,
        pub smoke_color: u32
    } = {
        Self::new()
    }
}


impl GameState {
    fn new() -> Self {
        // let mut grid: Vec<Vec<CELLVAL>> = createBlankGrid();
        // let wallSpawns: Vec<(usize, usize)> = vec![(3, 1)];
        // let borders: Vec<(usize, usize)> = createBorders();
        // let wallSpawns: Vec<(usize, usize)> = vec![(1,1), (2,1), (3,1), (15,1), (16,1),
        // (9,2), (11,2),
        // (3,3), (4,3), (6,3), (7,3), (8,3), (9,3), (11,3), (12,3), (15,3),
        // (4,4), (11,4), (15,4),
        // (3,5), (4,5), (9,5), (11,5),
        // (1,6), (7,6), (9,6), (13,6), (14,6),
        // (1,7), (5,7), (6,7), (7,7), (9,7), (10,7), (11,7)
        // ];

        // for wallTuple in borders {
        //     grid[wallTuple.1][wallTuple.0] = CELLVAL::Wall;
        //     // sprite!(
        //     //     "dumpster", x = wallTuple.0 * CELL_SIZE, y = wallTuple.1 * CELL_SIZE
        //     // );
        // }
        // for wallTuple in wallSpawns {
        //     grid[wallTuple.1][wallTuple.0] = CELLVAL::Wall;
        // }


        Self {
            // grid,
            P1Char: PlayerCharacter::new("player1".to_string(), 0),
            move_delay_timer: 0.0,
            screenshake_timer: 0.0,
            smoke_color: 0xffffff50,
            smoke_timer: 0.0
        }
    }
}

turbo::go!({
    let mut state = GameState::load();

    clear!(0xADD8E6FF);
    // let (x, y, w, h) = (36, 102, 60, 20);
    // let mut color = 0x00008BFF;

    // let m = mouse(0);
    // //check if mouse is over the button and clicked
    // if (m.position[0] >= x && m.position[0] <= x + w)
    //     && (m.position[1] >= y && m.position[1] <= y + h)
    // {
    //     color = 0x4169E1FF;
    //     if m.left.just_pressed() {
    //         os::client::exec(env::PROJECT_NAME, "hello", &[]);
    //     }
    // }
    // //draw a button
    // rect!(x = x, y = y, w = w, h = h, color = color, border_radius = 8);
    // text!("HELLO!!", x = 50, y = 109);

    // for y in 0..constants::MAP_DIM_Y {
    //     for x in 0..constants::MAP_DIM_X {
    //         match state.grid[y][x] {
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
    client::render();



    //screenshake

    //test input - convert to successful movement_end event
    // if gamepad(0).left.just_pressed() {
    //     state.screenshake_timer = (8 * 60 / 60) as f32;
    // }

    //shake screen for timer duration of screenshake
    if state.screenshake_timer > 0.0 {
        set_camera(rand() as i32 % 8, rand() as i32 % 8);
        state.screenshake_timer -= 1.0;
    } else {
        set_camera(0, 0);
    }


    //test input - convert to successful movement_start event
    if gamepad(0).left.just_pressed() {
         state.move_delay_timer = (0.4 * 60.0) as f32;
         //disable incoming damage temporarily
    }

    //delay movement_end event for timer duration of move_delay
    if state.move_delay_timer > 0.0 {
         state.move_delay_timer -= 1.0;
    } else {
        //execute movement code here
        //re-enable incoming damage
        state.smoke_timer = (1.0 * 60.0) as f32;
    }



    //delay movement_end event for timer duration of move_delay
    if state.smoke_timer > 0.0 {
         state.smoke_timer -= 1.0;
         
         //replace this with actual location of player
         let player_location = (160,160);

         let smoke_color = state.smoke_color;
         let edge_count = 32;
         let circle_radius = 16.0;
         let angle_offset = (tick() as f32) * 0.05;
         //smoke effect
         for i in 0..edge_count {
            let progress = i as f32 / edge_count as f32;
            let x = progress * (player_location.0 as f32);
            let y = (angle_offset + progress * PI * 2.0).sin() * 5.0;
            circ!(
                x = x - circle_radius,
                y = y - circle_radius,
                d = circle_radius * 2.0,
                color = smoke_color,
                // absolute = true
            );
        }
        // Draw bottom edge
        for i in 0..edge_count {
            let progress = i as f32 / edge_count as f32;
            let x = progress * (player_location.0 as f32);
            let y = (player_location.1 as f32) - ((angle_offset + progress * PI * 2.0).sin() * 5.0);
            circ!(
                x = x - circle_radius,
                y = y - circle_radius * 0.5,
                d = circle_radius * 2.0,
                color = smoke_color,
                // absolute = true
            );
        }
        // Draw left edge
        for i in 0..edge_count {
            let progress = i as f32 / edge_count as f32;
            let y = progress * (player_location.1 as f32);
            let x = (angle_offset + progress * PI * 2.0).cos() * 5.0;
            circ!(
                x = x - circle_radius * 1.5,
                y = y - circle_radius * 0.5,
                d = circle_radius * 2.0,
                color = smoke_color,
                // absolute = true
            );
        }
        // Draw right edge
        for i in 0..edge_count {
            let progress = i as f32 / edge_count as f32;
            let y = progress * (player_location.1 as f32);
            let x = (player_location.0 as f32) - ((angle_offset + progress * PI * 2.0).cos() * 5.0);
            circ!(
                x = x - circle_radius * 0.5,
                y = y - circle_radius * 0.5,
                d = circle_radius * 2.0,
                color = smoke_color,
                // absolute = true
            );
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