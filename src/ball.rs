use crate::config;
use crate::player::Player;
use crate::utils::color_to_hex_str;
use raylib::prelude::{Rectangle, Sound, Texture2D, Vector2};

///
/// Particle structure with basic data
///
#[derive(Debug, Default, Copy, Clone)]
pub struct BallTailParticle {
    pub position: Vector2,
    // Color color;
    pub alpha: f32,
    pub size: f32,
    // Use it to activate/deactive particle
    pub active: bool,
}

///
/// The lighting tail that follows by the moving ball
///
pub struct BallLightingTail {
    pub particles: [BallTailParticle; config::BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT],
}

///
///
///
pub struct Ball {
    pub center: Vector2,
    pub radius: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub lightning_ball_rotation_angle: f32,
    pub current_hits: usize,
    pub current_velocities_increase: usize,
    pub enabled_fireball: bool,
    pub enabled_lightning_ball: bool,
    pub alpha_mask: Texture2D,
    pub lightning_ball: Texture2D,
    pub enable_fireball_sound_effect: Sound,
    pub enable_lightning_ball_sound_effect: Sound,
    pub hit_racket_sound_effect: Sound,
    pub lighting_tail: BallLightingTail,
}

impl std::fmt::Debug for Ball {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[ Ball ] {{\n\tcenter: {{ x: {:.2}, y: {:.2} }},\n\tradius: {:.2}\n\tcolor: {}\n\tfireball color: {}\n\tvelocity x: {:.2}\n\tvelocity y: {:.2}\n\thits_before_increase_velocity: {}\n\tvelocities_increase_to_enable_fireball: {}\n\tvelocities_increase_to_enable_lightning_ball: {}\n\tvelocity_acceleration: {:.2}\n\tlighting_tail_particle_count: {}\n\tlighting_tail_particle_init_alpha: {:.2}\n\tlighting_tail_particle_size: {:.2}\n}}",
            self.center.x,
            self.center.y,
            self.radius,
            color_to_hex_str(&config::BALL_UI_BALL_COLOR),
            color_to_hex_str(&config::BALL_UI_FIREBALL_COLOR),
            config::BALL_UI_BALL_VELOCITY_X,
            config::BALL_UI_BALL_VELOCITY_Y,
            config::BALL_UI_HITS_BEFORE_INCREASE_VELOCITY,
            config::BALL_UI_VELOCITIES_INCREASE_TO_ENABLE_FIREBALL,
            config::BALL_UI_VELOCITIES_INCREASE_TO_ENABLE_LIGHTNING_BALL,
            config::BALL_UI_VELOCITY_ACCELERATION,
             config::BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT,
             config::BALL_UI_LIGHTING_TAIL_PRATICLE_INIT_ALPHA,
             config::BALL_UI_LIGHTING_TAIL_PRATICLE_SIZE,
        )
    }
}

impl Ball {
    ///
    ///
    ///
    pub fn redraw(&self) {}

    ///
    ///
    ///
    fn restart(&self, table_rect: &Rectangle) {}

    ///
    ///
    ///
    fn update(
        &mut self,
        table_rect: &Rectangle,
        player1: &Player,
        player2: &Player,
        is_player1_win: &mut bool,
        is_player2_win: &mut bool,
    ) {
    }

    ///
    ///
    ///
    fn update_lighting_tail(&mut self) {}
}
