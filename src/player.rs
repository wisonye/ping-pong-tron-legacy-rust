use raylib::prelude::{Color, Rectangle, Texture2D};

///
///
///
pub enum PlayerType {
    Left,
    Right,
}

///
///
///
enum RacketUpdateType {
    MoveUp,
    MoveDown,
    Reset,
}

///
///
///
pub struct Racket {
    color: Color,
    rect: Rectangle,
    rect_texture: Texture2D,
}

///
///
///
pub struct Player {
    r#type: PlayerType,
    name: String,
    score: usize,
    level: usize,
    // The default one
    default_racket: Racket,
}

impl Player {
    ///
    ///
    ///
    fn win(&self) {}

    ///
    ///
    ///
    fn racket_redraw(&self, container: &Rectangle) {}

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
