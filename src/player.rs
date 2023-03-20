use crate::config;
use crate::utils::color_to_hex_str;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, Rectangle, Texture2D, Vector2};

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
enum RacketUpdateType {
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
    fn Player_update_racket(
        &self,
        container: &Rectangle,
        is_fullscreen: bool,
        rut: RacketUpdateType,
    ) {
    }
}
