use crate::ball::{Ball, BallLightingTail, BallTailParticle};
use crate::config;
use crate::player::{Player, PlayerType, Racket};
use crate::scoreboard::Scoreboard;
use crate::table::Table;
use raylib::prelude::{
    consts::{KeyboardKey, TraceLogLevel},
    logging::{set_trace_log, trace_log},
    window, Color, Image, RaylibAudio, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread,
    Rectangle, Sound, Vector2,
};
use std::env;
use std::str::FromStr;

///
///
///
#[derive(Debug)]
enum GameState {
    UnInit,
    Init,
    BeforeStart,
    Player,
    PlayerWins(PlayerType, String, usize),
    Pause,
}

///
///
///
#[derive(Debug)]
struct MiscSettings {
    game_fps: f32,
}

///
///
///
pub struct Game {
    pub rl_handle: RaylibHandle,
    pub rl_thread: RaylibThread,
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

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "{}", self.xxx)
        f.debug_struct("[ Game ]")
            .field("state", &self.state)
            .field("player1", &self.player1)
            .field("player2", &self.player2)
            .field("ball", &self.ball)
            .finish()
    }
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

        // You can't use `let _` here, otherwise, "failed to load sound XXX"
        let mut rl_audio = RaylibAudio::init_audio_device();

        // Hide the cursor
        rl_handle.hide_cursor();

        // Set our game FPS (frames-per-second)
        rl_handle.set_target_fps(config::GAME_FPS);

        // Set log level
        set_trace_log(TraceLogLevel::LOG_DEBUG);

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

        // game.print_debug_info();

        // trace_log(
        //     TraceLogLevel::LOG_DEBUG,
        //     ">>> [ Game_init ] - Game initialization [ done ]",
        // );

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
    pub fn redraw(&mut self, rdl: &mut RaylibDrawHandle) {}

    ///
    ///
    ///
    pub fn run(&mut self) {
        trace_log(
            TraceLogLevel::LOG_DEBUG,
            ">>> [ Game_run ] - Game is running......",
        );

        let screen_width = self.rl_handle.get_screen_width();
        let screen_height = self.rl_handle.get_screen_height();
        let default_font = self.rl_handle.get_font_default();

        while !self.rl_handle.window_should_close() {
            // if self.rl_handle.is_key_pressed(KeyboardKey::KEY_P) {
            //     if you_win_sound_effect.is_ok() {
            //         rl_audio.play_sound_multi(you_win_sound_effect.as_ref().unwrap());
            //     }
            // }

            //
            // Update game logic
            //
            // self.logic();

            let mut d = self.rl_handle.begin_drawing(&self.rl_thread);

            //
            // Clean last frame
            //
            d.clear_background(config::GAME_UI_BACKGROUND_COLOR);

            //
            // Redraw the entire game
            //

            //
            // Scoreboard
            //
            let sb_rect =
                self.scoreboard
                    .redraw(&mut d, screen_width, screen_height, &default_font);

            //
            // Table
            //
            // let table_rect = Table::redraw(&self, &sb_rect);

            // //
            // // Player rackets
            // //
            // self.player1.racket_redraw(&self.table_rect);
            // self.player2.racket_redraw(&self.table_rect);

            //
            // Ball
            //
            // self.ball.redraw();
            // //
            // // Update `game->table_rect` if changed
            // //
            // // trace_log(TraceLogLevel::LOG_DEBUG,
            // //          ">>> [ Game_redraw ] - table_rect: {x: %.2f, y: %.2f, width: "
            // //          "%.2f, height: %.2f}",
            // //          table_rect.x, table_rect.y, table_rect.width,
            // //          table_rect.height);
            // if table_rect.x != self.table_rect.x
            //     || table_rect.y != self.table_rect.y
            //     || table_rect.width != self.table_rect.width
            //     || table_rect.height != self.table_rect.height
            // {
            //     self.table_rect = table_rect;

            //     trace_log(
            //         TraceLogLevel::LOG_DEBUG,
            //         ">>> [ Game_redraw ] - Update 'game->table_rect'",
            //     );
            // }
        }

        trace_log(
            TraceLogLevel::LOG_DEBUG,
            ">>> [ Game_run ] - Exit the game loop",
        );
    }

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
    pub fn print_debug_info(&self) {
        println!("\n>>> [ print_debug_info ] - Game: {:#?}", self);

        //
        // `trace_log` has a bug, it CANNOT print too long string!!!!
        //
        // trace_log(TraceLogLevel::LOG_DEBUG, debug_info.as_str());
    }
}
