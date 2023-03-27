// use raylib::prelude::{
//     consts::{KeyboardKey, TraceLogLevel},
//     logging::{set_trace_log, trace_log},
//     Color, Image, RaylibAudio, RaylibDraw, Sound,
// };
mod ball;
mod config;
mod game;
mod player;
mod scoreboard;
mod utils;

use game::Game;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match Game::init() {
        Ok(mut game) => game.run(),
        Err(e) => println!("\n>>> [ Game init fail ]: {:?}", e),
    }

    // let init_result = Game::init();
    // if let Ok(mut game) = init_result {
    //     game.run();

    // } else {
    //     println!("\n>>> [ Game init fail ]: {:?}", init_result.err());
    // }

    // let (mut rl_handle, rl_thread) = raylib::init()
    //     .size(
    //         config::GAME_UI_INIT_SCREEN_WIDTH,
    //         config::GAME_UI_INIT_SCREEN_HEIGHT,
    //     )
    //     .title(config::GAME_UI_TITLE)
    //     .undecorated()
    //     .build();

    // let mut rl_audio = RaylibAudio::init_audio_device();

    // // Hide the cursor
    // rl_handle.hide_cursor();

    // // Set our game FPS (frames-per-second)
    // rl_handle.set_target_fps(config::GAME_FPS);

    // // Set log level
    // set_trace_log(TraceLogLevel::LOG_DEBUG);

    // //
    // // Load sound effects
    // //
    // let enable_fireball_sound_effect = Sound::load_sound(config::ENABLE_FIREBALL_SOUND_EFFECT)?;
    // let you_win_sound_effect = Sound::load_sound(config::YOU_WIN_SOUND_EFFECT_2)?;
    // let enable_lightning_ball_sound_effect =
    //     Sound::load_sound(config::ENABLE_LIGHTNING_BALL_SOUND_EFFECT)?;
    // let hit_racket_sound_effect = Sound::load_sound(config::BALL_HIT_RACKET_SOUND_EFFECT)?;

    // while !rl_handle.window_should_close() {
    //     let mut d = rl_handle.begin_drawing(&rl_thread);

    //     d.clear_background(Color::WHITE);
    //     d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    // }

    Ok(())
}
