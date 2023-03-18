use raylib::prelude::*;

mod ball;
mod config;
mod game;
mod player;
mod scoreboard;
mod table;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Ping pong tron legacy")
        .undecorated()
        .build();

    let mut rl_audio = RaylibAudio::init_audio_device();

    //
    // Load sound effects
    //
    // game->you_win_sound_effect = LoadSound(YOU_WIN_SOUND_EFFECT_1);
    let you_win_sound_effect = Sound::load_sound(config::YOU_WIN_SOUND_EFFECT_2);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            if you_win_sound_effect.is_ok() {
                rl_audio.play_sound_multi(you_win_sound_effect.as_ref().unwrap());
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
