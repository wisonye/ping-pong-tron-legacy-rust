use crate::utils::color_to_hex_str;
use crate::config;
use raylib::prelude::{Color, Rectangle, Texture2D};

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
    fn win(&self) {}

    ///
    ///
    ///
    pub fn racket_redraw(&self, container: &Rectangle) {}

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
