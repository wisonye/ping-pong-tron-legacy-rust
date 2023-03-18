use crate::ball::{Ball, BallLightingTail, BallTailParticle};
use crate::config;
use crate::player::{Player, PlayerType, Racket};
use crate::scoreboard::Scoreboard;
use raylib::prelude::{
    window, Color, Image, RaylibAudio, RaylibHandle, RaylibThread, Rectangle, Sound, Vector2,
};
use std::env;
use std::str::FromStr;

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
pub struct Game {
    rl_handle: RaylibHandle,
    rl_thread: RaylibThread,
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
    /// Load related game resources to create `Game` instance and init game
    /// window
    ///
    pub fn init() -> Result<Self, String> {
        let (mut rl_handle, rl_thread) = raylib::init()
            .size(
                config::GAME_UI_INIT_SCREEN_WIDTH,
                config::GAME_UI_INIT_SCREEN_HEIGHT,
            )
            .title(config::GAME_UI_TITLE)
            .undecorated()
            .build();

        let mut rl_audio = RaylibAudio::init_audio_device();

        // Hide the cursor
        rl_handle.hide_cursor();

        // Set our game FPS (frames-per-second)
        rl_handle.set_target_fps(config::GAME_FPS);

        //
        // Load sound effects
        //
        // let you_win_sound_effect = Sound::load_sound(config::YOU_WIN_SOUND_EFFECT_1);
        let you_win_sound_effect = Sound::load_sound(config::YOU_WIN_SOUND_EFFECT_2)?;
        let enable_fireball_sound_effect = Sound::load_sound(config::ENABLE_FIREBALL_SOUND_EFFECT)?;
        let enable_lightning_ball_sound_effect =
            Sound::load_sound(config::ENABLE_LIGHTNING_BALL_SOUND_EFFECT)?;
        let hit_racket_sound_effect = Sound::load_sound(config::BALL_HIT_RACKET_SOUND_EFFECT)?;

        //
        // As I want to draw the ball with gradient visual effects (like a halo)
        // and a lighting trail that follows the moving ball, that's why do I need
        // to create an alpha mask image (with black and white color) as the
        // blending mask.
        //
        // - The `density` affects the halo border length!!!
        //
        // - The size of the alpha mask must be the same size of the ball
        //
        // - The lighting tail is just a bunch of particle instances, each particle
        //   has the init alpha value and size, and the size should be smaller than
        //   the ball to make it looks nicer.
        //
        let density: f32 = 0.5;
        let ball_radius = match env::var("BALL_RADIUS") {
            Ok(radius) => f32::from_str(&radius).unwrap(),
            Err(_) => config::BALL_UI_BALL_RADIUS,
        };

        // printf("\n\n>>> ball_radisu: %f\n\n", ball_radius);
        let ball_alpha_mask_image = Image::gen_image_gradient_radial(
            ball_radius as i32 * 2,
            ball_radius as i32 * 2,
            density,
            Color::WHITE,
            Color::BLACK,
        );

        //
        // Lightning ball
        //
        let lightning_ball_image = Image::load_image(config::BALL_UI_LIGHTNING_BALL)?;

        //
        // Racket gradient texture
        //
        let mut racket_image = Image::load_image(config::RACKET_UI_LASER_RACKET_TEXTURE)?;
        racket_image.resize(config::RACKET_UI_WIDTH, config::RACKET_UI_HEIGHT);
        let player1_default_racket_rect_texture =
            rl_handle.load_texture_from_image(&rl_thread, &racket_image)?;
        let player2_default_racket_rect_texture =
            rl_handle.load_texture_from_image(&rl_thread, &racket_image)?;

        // rl_hanlde.trace_log(LOG_DEBUG, ">>> [ Game_init ] - Game initialization [ done ]");

        let player1 = Player {
            r#type: PlayerType::Left,
            name: match env::var("PLAYER_1_NAME") {
                Ok(value) => value,
                Err(_) => config::PLAYER_1_NAME.to_string(),
            },
            score: 0,
            level: 0,
            // The default one
            default_racket: Racket {
                color: config::RACKET_UI_COLOR,
                rect: Rectangle::default(),
                rect_texture: player1_default_racket_rect_texture,
            },
        };

        let player2 = Player {
            r#type: PlayerType::Left,
            name: match env::var("PLAYER_2_NAME") {
                Ok(value) => value,
                Err(_) => config::PLAYER_2_NAME.to_string(),
            },
            score: 0,
            level: 0,
            // The default one
            default_racket: Racket {
                color: config::RACKET_UI_COLOR,
                rect: Rectangle::default(),
                rect_texture: player2_default_racket_rect_texture,
            },
        };

        //
        let ball = Ball {
            center: Vector2 { x: -1.0, y: -1.0 },
            radius: ball_radius,
            velocity_x: config::BALL_UI_BALL_VELOCITY_X,
            velocity_y: config::BALL_UI_BALL_VELOCITY_Y,
            lightning_ball_rotation_angle: 0.0,
            current_hits: 0,
            current_velocities_increase: 0,
            enabled_fireball: false,
            enabled_lightning_ball: false,

            //
            // `alpha_mask` is a black and white color image that uses for
            // blending operations, it HAS TO be created after the
            // `InitWindow` call. That means it creates inside
            // `Game_init()`, not here!!!
            //
            alpha_mask: rl_handle.load_texture_from_image(&rl_thread, &ball_alpha_mask_image)?,
            lightning_ball: rl_handle.load_texture_from_image(&rl_thread, &lightning_ball_image)?,
            enable_fireball_sound_effect,
            enable_lightning_ball_sound_effect,
            hit_racket_sound_effect,
            lighting_tail: BallLightingTail {
                particles: [BallTailParticle::default();
                    config::BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT],
            },
        };

        let game = Self {
            rl_handle,
            rl_thread,
            scoreboard: Scoreboard::new(&player1.name, &player2.name),
            player1,
            player2,
            table_rect_before_screen_changed: Rectangle::default(),
            table_rect: Rectangle::default(),
            ball,
            state: GameState::BeforeStart, // Set to `GS_BEFORE_START`
            is_fullscreen: false,
            is_player1_wins_last_round: false,
            you_win_sound_effect,
        };

        Ok(game)
    }

    ///
    ///
    ///
    pub fn get_player1(&self) -> &Player {
        &self.player1
    }

    ///
    ///
    ///
    pub fn get_player2(&self) -> &Player {
        &self.player2
    }
    ///
    ///
    ///
    fn toggle_fullscreen(&mut self) {
        if !self.is_fullscreen {
            let monitor = window::get_current_monitor();
            self.rl_handle.set_window_size(
                window::get_monitor_width(monitor),
                window::get_monitor_height(monitor),
            );
            self.rl_handle.toggle_fullscreen();
            self.is_fullscreen = true;
        } else {
            self.rl_handle.toggle_fullscreen();
            self.rl_handle.set_window_size(
                config::GAME_UI_INIT_SCREEN_WIDTH,
                config::GAME_UI_INIT_SCREEN_HEIGHT,
            );
            self.is_fullscreen = false;
        }
    }
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
