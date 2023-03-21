use crate::config;
use crate::player::Player;
use crate::utils::color_to_hex_str;
use raylib::prelude::{
    Color, RaylibBlendMode, RaylibDraw, RaylibDrawHandle, Rectangle, Sound, Texture2D, Vector2,
};

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
    pub fn redraw(&self, blend_mode_handle: &mut RaylibBlendMode<RaylibDrawHandle>) {
        if self.center.x == -1.0 || self.center.y == -1.0 {
            return;
        }

        //
        // Draw lighting tail
        //
        let particles = &self.lighting_tail.particles;

        let mut ball_and_lighting_tail_color = if self.enabled_fireball {
            config::BALL_UI_FIREBALL_COLOR
        } else {
            config::BALL_UI_BALL_COLOR
        };

        if self.enabled_lightning_ball {
            ball_and_lighting_tail_color = config::BALL_UI_LIGHTNING_BALL_COLOR;
        }

        for i in 0..config::BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT {
            if self.lighting_tail.particles[i].active {
                blend_mode_handle.draw_texture_pro(
                    &self.alpha_mask,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: self.alpha_mask.width as f32,
                        height: self.alpha_mask.height as f32,
                    },
                    Rectangle {
                        x: particles[i].position.x,
                        y: particles[i].position.y,
                        width: self.alpha_mask.width as f32 * particles[i].size,
                        height: self.alpha_mask.height as f32 * particles[i].size,
                    },
                    Vector2 {
                        x: (self.alpha_mask.width as f32 * particles[i].size / 2.0),
                        y: (self.alpha_mask.height as f32 * particles[i].size / 2.0),
                    },
                    0.0,
                    Color::fade(&ball_and_lighting_tail_color, particles[i].alpha),
                );
            }
        }

        //
        // Draw solid circle
        //
        // DrawCircleV(self.center, self.radius, self.color);

        //
        // Draw ball with alpha mask
        //
        blend_mode_handle.draw_texture_pro(
            &self.alpha_mask,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: self.alpha_mask.width as f32,
                height: self.alpha_mask.height as f32,
            },
            Rectangle {
                x: self.center.x,
                y: self.center.y,
                width: self.alpha_mask.width as f32,
                height: self.alpha_mask.height as f32,
            },
            Vector2 {
                x: self.alpha_mask.width as f32 / 2.0,
                y: self.alpha_mask.height as f32 / 2.0,
            },
            0.0,
            ball_and_lighting_tail_color,
        );

        //
        // Draw lightning ball with texture png version
        //
        // if (self.enabled_lightning_ball) {
        //     BeginBlendMode(BLEND_ALPHA);

        //     DrawTexturePro(
        //         self.lightning_ball,
        //         (Rectangle){0.0f, 0.0f, (float)self.lightning_ball.width,
        //                     (float)self.lightning_ball.height},
        //         (Rectangle){self.center.x, self.center.y, 2 * self.radius,
        //                     2 * self.radius},
        //         (Vector2){(float)(self.radius), (float)(self.radius)},
        //         self.lightning_ball_rotation_angle,
        //         (Color){.r = 0xFF, .g = 0xFF, .b = 0xFF, .a = 0xFF});

        //     EndBlendMode();
        // }
    }

    ///
    /// Reset the ball and lighting tail
    ///
    pub fn restart(&mut self, table_rect: &Rectangle) {
        self.center = Vector2 {
            x: table_rect.x + ((table_rect.width - self.radius) / 2.0),
            y: table_rect.y + ((table_rect.height - self.radius) / 2.0),
        };

        self.velocity_x = config::BALL_UI_BALL_VELOCITY_X;
        self.velocity_y = config::BALL_UI_BALL_VELOCITY_Y;
        self.current_hits = 0;
        self.current_velocities_increase = 0;
        self.enabled_fireball = false;
        self.enabled_lightning_ball = false;

        let mut particles = &mut self.lighting_tail.particles;

        for i in 0..config::BALL_UI_LIGHTING_TAIL_PARTICLE_COUNT {
            particles[i].position = Vector2 { x: 0.0, y: 0.0 };
            // particles[i].color = self.color;

            // Init `alpha` value, it affects how light the particle at the
            // beginning
            particles[i].alpha = config::BALL_UI_LIGHTING_TAIL_PRATICLE_INIT_ALPHA;

            // It affects how big the particle will be: how many percentage of the
            // ball size: 0.0 ~ 1.0 (0 ~ 100%)
            particles[i].size = config::BALL_UI_LIGHTING_TAIL_PRATICLE_SIZE;
            particles[i].active = false;
        }
    }

    ///
    ///
    ///
    pub fn update(
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
