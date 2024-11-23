mod environment;
use std::collections::HashMap;
use environment::Wall;
mod constants;

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
        grid: Vec<Vec<bool>>,
        walls: Vec<Wall>,
    } = {
        Self::new()
    }
}

impl GameState {
    fn new() -> Self {
        Self {
            grid: vec![],
            walls: vec![]
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
            os::client::exec("dash-pandas", "hello", &[]);
        }
    }
    //draw a button
    rect!(x = x, y = y, w = w, h = h, color = color, border_radius = 8);
    text!("HELLO!!", x = 50, y = 109);
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}