use crate::config;
use crate::player::Player;
use raylib::prelude::{Rectangle, Sound, Texture2D, Vector2};

///
/// Particle structure with basic data
///
#[derive(Default, Copy, Clone)]
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

impl Ball {
    ///
    ///
    ///
    fn redraw(&self) {}

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
