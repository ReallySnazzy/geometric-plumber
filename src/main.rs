use drawing::draw;
use game_logic::{state::initial_state, tick};
use num::{BigInt, FromPrimitive};
use raylib::{prelude::*};

mod assets;
mod game_logic;
mod drawing;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Geometric Plumber")
        .build();

    rl.set_target_fps(60);
    let loaded_assets = assets::load_assets(&mut rl, &thread);
    let mut state = initial_state();

    while !rl.window_should_close() {
        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            state.mouse_click_x = rl.get_mouse_x();
            state.mouse_click_y = rl.get_mouse_y();
        }
        if rl.is_key_released(KeyboardKey::KEY_E) {
            state.score = BigInt::from_u64(500_000_000_000_000_000).unwrap();
        }
        tick::tick(&mut state);
        draw(&mut rl, &thread, &state, &loaded_assets);
    }
}