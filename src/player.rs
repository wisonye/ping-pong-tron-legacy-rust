use crate::config;
use crate::utils::color_to_hex_str;
use raylib::prelude::{
    consts::{KeyboardKey, TraceLogLevel},
    logging::{set_trace_log, trace_log},
    Color, RaylibDraw, RaylibDrawHandle, Rectangle, Texture2D, Vector2,
};

///
///
///
#[derive(Debug, PartialEq)]
pub enum PlayerType {
    Left,
    Right,
}

///
///
///
#[derive(Debug)]
pub enum RacketUpdateType {
    MoveUp,
    MoveDown,
    Reset,
}

///
///
///
pub struct Racket {
    pub color: Color,
    pub rect: Rectangle,
    pub rect_texture: Texture2D,
}

impl std::fmt::Debug for Racket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[ Racket ] {{ color: {}, velocity: {:.2} }}",
            color_to_hex_str(&self.color).as_str(),
            config::RACKET_UI_VELOCITY
        )
    }
}

///
///
///
pub struct Player {
    pub r#type: PlayerType,
    pub name: String,
    pub score: usize,
    pub level: usize,
    // The default one
    pub default_racket: Racket,
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("[ Player ]")
            .field("type", &self.r#type)
            .field("name", &self.name)
            .field("score", &self.score)
            .field("default_racket", &self.default_racket)
            .field("default_racket", &self.default_racket)
            .finish()
    }
}

impl Player {
    ///
    ///
    ///
    fn win(&mut self) {
        self.score += 1;
    }

    ///
    ///
    ///
    pub fn racket_redraw(&self, rdl: &mut RaylibDrawHandle) {
        let racket_rect = self.default_racket.rect;

        // DrawRectangleRec(player->default_racket.rect,
        // player->default_racket.color);

        // BeginBlendMode(BLEND_ADDITIVE);
        rdl.draw_texture_pro(
            &self.default_racket.rect_texture,
            // Texture rect to draw from
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: config::RACKET_UI_WIDTH as f32,
                height: config::RACKET_UI_HEIGHT as f32,
            },
            // Target rect to draw (orgin is TopLeft by default!!!)
            racket_rect,
            // Origin offset of the target rect to draw (TopLeft by default)
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            config::RACKET_UI_COLOR,
        );
        // EndBlendMode();

        if config::RACKET_UI_DRAW_DEBUG_BOUNDARY {
            rdl.draw_rectangle_rec(
                self.default_racket.rect,
                Color::fade(&self.default_racket.color, 0.5),
            );
        }
    }

    ///
    ///
    ///
    fn update_racket_after_screen_size_changed(
        &self,
        container: &Rectangle,
        old_container: &Rectangle,
    ) {
    }

    ///
    ///
    ///
    pub fn update_racket(
        &mut self,
        container: &Rectangle,
        rut: RacketUpdateType,
        current_frame_time: f32,
    ) {
        match rut {
            //
            // Center `y`
            //
            RacketUpdateType::Reset => {
                println!("\n>>> player type: {:?}", self.r#type);
                self.default_racket.rect = Rectangle {
                    x: if self.r#type == PlayerType::Left {
                        container.x + config::RACKET_UI_MARGIN
                    } else {
                        container.x + container.width
                            - config::RACKET_UI_MARGIN as f32
                            - config::RACKET_UI_WIDTH as f32
                    },
                    y: container.y + ((container.height - config::RACKET_UI_HEIGHT as f32) / 2.0),
                    width: config::RACKET_UI_WIDTH as f32,
                    height: config::RACKET_UI_HEIGHT as f32,
                };
                trace_log(
                    TraceLogLevel::LOG_DEBUG,
                    "[ Player_update_racket ] - RUT_RESET",
                );
            }
            //
            // Apply velocity to `y`
            //
            RacketUpdateType::MoveUp => {
                let new_y =
                    self.default_racket.rect.y - config::RACKET_UI_VELOCITY * current_frame_time;
                if new_y >= container.y {
                    self.default_racket.rect.y = new_y;
                }
            }
            RacketUpdateType::MoveDown => {
                let new_y =
                    self.default_racket.rect.y + config::RACKET_UI_VELOCITY * current_frame_time;
                if new_y + config::RACKET_UI_HEIGHT as f32 <= container.y + container.height {
                    self.default_racket.rect.y = new_y;
                }
            }
        }
    }
}
