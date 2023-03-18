use raylib::prelude::Rectangle;

///
///
///
pub struct Scoreboard {
    player1_name: String,
    player2_name: String,
    player1_score: usize,
    player2_score: usize,
}

impl Scoreboard {
    ///
    ///
    ///
    pub fn new(player1_name: &str, player2_name: &str) -> Self {
        Self {
            player1_name: String::from(player1_name),
            player2_name: String::from(player2_name),
            player1_score: 0,
            player2_score: 0,
        }
    }

    ///
    ///
    ///
    pub fn reset_player_score_to_zero(&mut self) {
        self.player1_score = 0;
        self.player2_score = 0;
    }

    ///
    ///
    ///
    pub fn add_player1_score(&mut self) {
        self.player1_score += 1;
    }

    ///
    ///
    ///
    pub fn add_player2_score(&mut self) {
        self.player2_score += 1;
    }

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
