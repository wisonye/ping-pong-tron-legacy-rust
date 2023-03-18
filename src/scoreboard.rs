use crate::player::Player;
use raylib::prelude::Rectangle;

///
///
///
pub struct Scoreboard {
    player1: Player,
    player2: Player,
}

impl Scoreboard {
    ///
    ///
    ///
    fn redraw(&self) -> Rectangle {
        Rectangle::default()
    }

    ///
    ///
    ///
    fn recalculate_rect(&self) -> Rectangle {
        Rectangle::default()
    }
}
