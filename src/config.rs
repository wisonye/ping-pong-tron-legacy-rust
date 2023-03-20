use raylib::prelude::*;

//
//
// Player
pub const PLAYER_1_NAME: &'static str = "Player 1";
pub const PLAYER_2_NAME: &'static str = "Player 2";

//
// Color theme
//
pub const TRON_DARK: Color = Color::new(0x23, 0x21, 0x1B, 0xFF);
pub const TRON_LIGHT_BLUE: Color = Color::new(0xAC, 0xE6, 0xFE, 0xFF);
pub const TRON_BLUE: Color = Color::new(0x6F, 0xC3, 0xDF, 0xFF);
pub const TRON_YELLOW: Color = Color::new(0xFF, 0xE6, 0x4D, 0xFF);
pub const TRON_ORANGE: Color = Color::new(0xFF, 0x9F, 0x1C, 0xFF);
pub const TRON_RED: Color = Color::new(0xF4, 0x47, 0x47, 0xFF);

//
// Game misc settings
//
pub const GAME_FPS: u32 = 60;

//
// Game UI settings
//
pub const GAME_UI_TITLE: &'static str = "Ping pong tron legacy";
pub const GAME_UI_INIT_SCREEN_WIDTH: i32 = 1300;
pub const GAME_UI_INIT_SCREEN_HEIGHT: i32 = 768;
pub const GAME_UI_PADDING: f32 = 10.0;
pub const GAME_UI_BACKGROUND_COLOR: Color = TRON_DARK;
pub const GAME_UI_BORDER_COLOR: Color = TRON_LIGHT_BLUE;
pub const GAME_UI_RACKET_COLOR: Color = TRON_ORANGE;

//
// Scoreboard UI settings
//
pub const SCOREBOARD_UI_PADDING: f32 = GAME_UI_PADDING;
pub const SCOREBOARD_UI_BORDER_COLOR: Color = TRON_LIGHT_BLUE;
pub const SCOREBOARD_UI_BORDER_HEIGHT_PERCENT: f32 = 10.0;
pub const SCOREBOARD_UI_BORDER_THICKNESS: i32 = 2;
pub const SCOREBOARD_UI_VS_FONT_SIZE: f32 = 30.0;
pub const SCOREBOARD_UI_VS_FONT_SPACE: f32 = 10.0;
pub const SCOREBOARD_UI_PLAYER_FONT_SPACE: f32 = 5.0;
pub const SCOREBOARD_UI_PLAYER_NAME_FONT_SIZE: f32 = 30.0;
pub const SCOREBOARD_UI_PLAYER_SCORE_FONT_SIZE: f32 = 50.0;
pub const SCOREBOARD_UI_SPACE_BETWEEN_NAME_AND_BORDER: f32 = 50.0;
pub const SCOREBOARD_UI_SPACE_BETWEEN_NAME_AND_SCORE: f32 = 50.0;

//
// Table UI settings
//
pub const TABLE_UI_MARGIN: f32 = GAME_UI_PADDING;
pub const TABLE_UI_BORDER_COLOR: Color = TRON_LIGHT_BLUE;
pub const TABLE_UI_BORDER_THICKNESS: i32 = 2;
pub const TABLE_UI_FIRST_START_PROMPT_BORDER_COLOR: Color = TRON_ORANGE;
pub const TABLE_UI_FIRST_START_PROMPT_TEXT_COLOR: Color = TRON_ORANGE;
pub const TABLE_UI_FIRST_START_PROMPT_TEXT: &'static str = "Press 'Space' to start the game";
pub const TABLE_UI_FIRST_START_PROMPT_FONT_SIZE: f32 = 40.0;
pub const TABLE_UI_FIRST_START_PROMPT_FONT_SPACE: f32 = 5.0;
pub const TABLE_UI_FIRST_START_PROMPT_CONTAINER_HORIZONTAL_PADDING: f32 = 50.0;
pub const TABLE_UI_FIRST_START_PROMPT_CONTAINER_VERTICAL_PADDING: f32 = 20.0;
pub const TABLE_UI_PLAYER_WINS_PROMPT_BORDER_COLOR: Color = TRON_ORANGE;
pub const TABLE_UI_PLAYER_WINS_PROMPT_TEXT_COLOR: Color = TRON_ORANGE;
pub const TABLE_UI_PLAYER_WINS_PROMPT_TEXT: &'static str = " Wins!!!";
pub const TABLE_UI_PLAYER_WINS_PROMPT_FONT_SIZE: f32 = 60.0;
pub const TABLE_UI_PLAYER_WINS_PROMPT_FONT_SPACE: f32 = 5.0;
pub const TABLE_UI_PLAYER_WINS_PROMPT_CONTAINER_HORIZONTAL_PADDING: f32 = 60.0;
pub const TABLE_UI_PLAYER_WINS_PROMPT_CONTAINER_VERTICAL_PADDING: f32 = 50.0;
pub const TABLE_UI_PLAYER_WINS_RESTART_TEXT_COLOR: Color = TRON_ORANGE;
pub const TABLE_UI_PLAYER_WINS_RESTART_TEXT: &'static str = "Press 'Space' to start the game";
pub const TABLE_UI_PLAYER_WINS_RESTART_FONT_SIZE: f32 = 20.0;
pub const TABLE_UI_PLAYER_WINS_RESTART_FONT_SPACE: f32 = 2.5;
pub const TABLE_UI_PLAYER_WINS_RESTART_CONTAINER_HORIZONTAL_PADDING: f32 = 30.0;
pub const TABLE_UI_PLAYER_WINS_RESTART_CONTAINER_VERTICAL_PADDING: f32 = 10.0;

//
// Ball UI settings
//
pub const BALL_UI_BALL_COLOR: Color = TRON_LIGHT_BLUE;
pub const BALL_UI_FIREBALL_COLOR: Color = TRON_ORANGE;
pub const BALL_UI_LIGHTNING_BALL_COLOR: Color = TRON_YELLOW;
pub const BALL_UI_LIGHTNING_BALL: &'static str = "resources/lightning_ball.png";
pub const BALL_UI_BALL_RADIUS: f32 = 30.0; // 20.0f // 60.f
pub const BALL_UI_LIGHTING_BALL_RADIUS: f32 = 10.0; // 20.0f // 60.f
pub const BALL_UI_BALL_VELOCITY_X: f32 = 500.0;
pub const BALL_UI_BALL_VELOCITY_Y: f32 = 500.0;
// How many hits before increasing the ball velocity
pub const BALL_UI_HITS_BEFORE_INCREASE_VELOCITY: usize = 2;
// How many velocities increase to enable a fireball
pub const BALL_UI_VELOCITIES_INCREASE_TO_ENABLE_FIREBALL: usize = 4;
// How many velocities increase to enable a lightning ball
pub const BALL_UI_VELOCITIES_INCREASE_TO_ENABLE_LIGHTNING_BALL: usize = 6;
// Velocity acceleration
pub const BALL_UI_VELOCITY_ACCELERATION: f32 = 100.0;
pub const BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT: usize = 50;
// Init `alpha` value, it affects how light the particle at the beginning
pub const BALL_UI_LIGHTING_TAIL_PRATICLE_INIT_ALPHA: f32 = 0.8;
// It affects how big the particle will be: how many percentage of the ball
// size: 0.0 ~ 1.0 (0 ~ 100%)
pub const BALL_UI_LIGHTING_TAIL_PRATICLE_SIZE: f32 = 0.5;
pub const BALL_UI_LIGHTING_TAIL_PRATICLE_SIZE_FOR_LIGHTNING_BALL: f32 = 0.4;

//
// Racket UI settings
//
pub const RACKET_UI_MAX_RACKETS_PER_PLAYER: usize = 5;
pub const RACKET_UI_MARGIN: f32 = 20.0;
pub const RACKET_UI_WIDTH: i32 = 40;
pub const RACKET_UI_HEIGHT: i32 = 200;
pub const RACKET_UI_COLOR: Color = TRON_LIGHT_BLUE;
pub const RACKET_UI_VELOCITY: f32 = 600.0;
pub const RACKET_UI_DRAW_DEBUG_BOUNDARY: bool = false;
// pub const  RACKET_UI_LASER_RACKET_TEXTURE "resources/green_laser.png"
pub const RACKET_UI_LASER_RACKET_TEXTURE: &'static str = "resources/blue_laser.png";

//
// Player settings
//
pub const PLAYER_1_UP_KEY: KeyboardKey = KeyboardKey::KEY_E;
pub const PLAYER_1_DOWN_KEY: KeyboardKey = KeyboardKey::KEY_D;
pub const PLAYER_2_UP_KEY: KeyboardKey = KeyboardKey::KEY_K;
pub const PLAYER_2_DOWN_KEY: KeyboardKey = KeyboardKey::KEY_J;

//
// Sound effects
//
pub const ENABLE_FIREBALL_SOUND_EFFECT: &'static str = "resources/enable_fireball.wav";
pub const ENABLE_LIGHTNING_BALL_SOUND_EFFECT: &'static str = "resources/enable_lightning_ball.wav";
pub const BALL_HIT_RACKET_SOUND_EFFECT: &'static str = "resources/hit_racket.wav";
pub const YOU_WIN_SOUND_EFFECT_1: &'static str = "resources/you_win.wav";
pub const YOU_WIN_SOUND_EFFECT_2: &'static str = "resources/you_win_2.wav";
