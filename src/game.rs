use crate::ball::Ball;
use crate::player::Player;
use crate::scoreboard::Scoreboard;
use raylib::prelude::{Rectangle, Sound};

///
///
///
enum GameState {
    UnInit = 0x01,
    Init = 0x02,
    BeforeStart = 0x03,
    Player = 0x04,
    PlayerWins = 0x05,
    Pause = 0x06,
}

///
///
///
struct MiscSettings {
    game_fps: f32,
}

///
///
///
struct Game {
    player1: Player,
    player2: Player,
    scoreboard: Scoreboard,
    table_rect_before_screen_changed: Rectangle,
    table_rect: Rectangle,
    ball: Ball,
    state: GameState,
    is_fullscreen: bool,
    is_player1_wins_last_round: bool,
    you_win_sound_effect: Sound,
}

impl Game {
    ///
    ///
    ///
    pub fn init(&self) {}

    ///
    ///
    ///
    pub fn redraw(&self) {}

    ///
    ///
    ///
    pub fn run(&self) {}

    ///
    ///
    ///
    pub fn pause(&self) {}

    ///
    ///
    ///
    pub fn resume(&self) {}

    ///
    ///
    ///
    pub fn print_debug_info(&self) {}
}
