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


turbo::go!({
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

    if gamepad(0).left.pressed() {
        os::client::exec("dash-pandas", "input_left", &[]);
    }
    if gamepad(0).right.pressed() {
        os::client::exec("dash-pandas", "input_right", &[]);
    }
    if gamepad(0).up.pressed() {
        os::client::exec("dash-pandas", "input_up", &[]);
    }
    if gamepad(0).down.pressed() {
        os::client::exec("dash-pandas", "input_down", &[]);
    }
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