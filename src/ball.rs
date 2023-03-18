use crate::config;
use crate::player::Player;
use raylib::prelude::{Rectangle, Sound, Texture2D, Vector2};

///
/// Particle structure with basic data
///
struct BallTailParticle {
    position: Vector2,
    // Color color;
    alpha: f32,
    size: f32,
    // Use it to activate/deactive particle
    active: bool,
}

///
/// The lighting tail that follows by the moving ball
///
struct BallLightingTail {
    particles: [BallTailParticle; config::BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT],
}

///
///
///
pub struct Ball {
    center: Vector2,
    radius: f32,
    velocity_x: f32,
    velocity_y: f32,
    lightning_ball_rotation_angle: f32,
    current_hits: usize,
    current_velocities_increase: usize,
    enabled_fireball: bool,
    enabled_lightning_ball: bool,
    alpha_mask: Texture2D,
    lightning_ball: Texture2D,
    enable_fireball_sound_effect: Sound,
    enable_lightning_ball_sound_effect: Sound,
    hit_racket_sound_effect: Sound,
    lighting_tail: BallLightingTail,
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
