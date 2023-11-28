use enigo::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();

    let time = Duration::from_secs(1);

    enigo.mouse_move_to(0, 0);
    sleep(time);
    enigo.mouse_click(MouseButton::Left);
    sleep(time);
    enigo.mouse_move_to(20, 240);
    sleep(time);
    enigo.key_down(Key::Option);
    sleep(time);
    enigo.mouse_click(MouseButton::Left);
}
