use raylib::prelude::*;

mod ball;
mod config;
mod game;
mod player;
mod scoreboard;
mod table;

use game::Game;

fn main() {
    let _ = Game::init();

    // while !rl.window_should_close() {
    //     if rl.is_key_pressed(KeyboardKey::KEY_P) {
    //         if you_win_sound_effect.is_ok() {
    //             rl_audio.play_sound_multi(you_win_sound_effect.as_ref().unwrap());
    //         }
    //     }

    //     let mut d = rl.begin_drawing(&thread);

    //     d.clear_background(Color::WHITE);
    //     d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    // }
}
