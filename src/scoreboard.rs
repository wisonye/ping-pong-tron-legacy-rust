use crate::config;
use raylib::prelude::{
    measure_text_ex, Color, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2, WeakFont,
};

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
    fn draw_player_name_and_score(
        rdl: &mut RaylibDrawHandle,
        name: &str,
        score: usize,
        is_player_1: bool,
        font: &WeakFont,
        container: &Rectangle,
        color: &Color,
    ) {
        //
        // Name
        //
        let name_font_size = measure_text_ex(
            font,
            name,
            config::SCOREBOARD_UI_PLAYER_NAME_FONT_SIZE,
            config::SCOREBOARD_UI_PLAYER_FONT_SPACE,
        );
        let name_point = Vector2 {
            x: if is_player_1 {
                container.x + config::SCOREBOARD_UI_SPACE_BETWEEN_NAME_AND_BORDER
            } else {
                container.x + container.width
                    - config::SCOREBOARD_UI_SPACE_BETWEEN_NAME_AND_BORDER
                    - name_font_size.x
            },
            y: container.y + ((container.height - name_font_size.y) / 2.0),
        };

        rdl.draw_text_ex(
            font,
            name,
            name_point,
            config::SCOREBOARD_UI_PLAYER_NAME_FONT_SIZE,
            config::SCOREBOARD_UI_PLAYER_FONT_SPACE,
            color,
        );

        //
        // Score (double digits)
        //
        let score_str = score.to_string();

        let score_font_size = measure_text_ex(
            font,
            &score_str,
            config::SCOREBOARD_UI_PLAYER_SCORE_FONT_SIZE,
            config::SCOREBOARD_UI_PLAYER_FONT_SPACE,
        );
        let score_font_point = Vector2 {
            x: if is_player_1 {
                name_point.x + name_font_size.x + config::SCOREBOARD_UI_SPACE_BETWEEN_NAME_AND_SCORE
            } else {
                name_point.x
                    - config::SCOREBOARD_UI_SPACE_BETWEEN_NAME_AND_SCORE
                    - score_font_size.x
            },
            y: container.y + ((container.height - score_font_size.y) / 2.0),
        };
        rdl.draw_text_ex(
            font,
            &score_str,
            score_font_point,
            config::SCOREBOARD_UI_PLAYER_SCORE_FONT_SIZE,
            config::SCOREBOARD_UI_PLAYER_FONT_SPACE,
            color,
        );
    }

    ///
    ///
    ///
    pub fn redraw(
        &self,
        rdl: &mut RaylibDrawHandle,
        screen_width: i32,
        screen_height: i32,
        default_font: &WeakFont,
    ) -> Rectangle {
        //
        // Outside border
        //
        let rect = Rectangle {
            x: config::SCOREBOARD_UI_PADDING,
            y: config::SCOREBOARD_UI_PADDING,
            width: screen_width as f32 - (2.0 * config::SCOREBOARD_UI_PADDING),
            height: screen_height as f32
                * (config::SCOREBOARD_UI_BORDER_HEIGHT_PERCENT / 100 as f32),
        };

        // TraceLog(LOG_DEBUG, ">>> [ SB_redraw ] - rect: %f, %f, %f, %f", rect.x,
        // rect.y, rect.width, rect.height);

        rdl.draw_rectangle_lines_ex(
            rect,
            config::SCOREBOARD_UI_BORDER_THICKNESS,
            config::SCOREBOARD_UI_BORDER_COLOR,
        );

        //
        // `VS`
        //
        let vs_font_size = measure_text_ex(
            &default_font,
            "VS",
            config::SCOREBOARD_UI_VS_FONT_SIZE,
            config::SCOREBOARD_UI_VS_FONT_SPACE,
        );
        // TraceLog(LOG_DEBUG, ">>> [ SB_redraw ] - vs_font_size: %f, %f",
        // vs_font_size.x, vs_font_size.y);

        let vs_font_draw_x = rect.x + ((rect.width - vs_font_size.x) / 2.0);
        let vs_font_draw_y = rect.y + ((rect.height - vs_font_size.y) / 2.0);
        let vs_font_point = Vector2 {
            x: vs_font_draw_x,
            y: vs_font_draw_y,
        };
        rdl.draw_text_ex(
            &default_font,
            "VS",
            vs_font_point,
            config::SCOREBOARD_UI_VS_FONT_SIZE,
            config::SCOREBOARD_UI_VS_FONT_SPACE,
            config::SCOREBOARD_UI_BORDER_COLOR,
        );
        // TraceLog(LOG_DEBUG, ">>> [ SB_redraw ] - vs_font_point: %f, %f",
        // vs_font_point.x, vs_font_point.y);

        //
        // Player
        //
        Self::draw_player_name_and_score(
            rdl,
            &self.player1_name,
            self.player1_score,
            true,
            &default_font,
            &rect,
            &config::SCOREBOARD_UI_BORDER_COLOR,
        );
        Self::draw_player_name_and_score(
            rdl,
            &self.player2_name,
            self.player2_score,
            false,
            &default_font,
            &rect,
            &config::SCOREBOARD_UI_BORDER_COLOR,
        );

        return rect;
    }

    ///
    ///
    ///
    fn recalculate_rect(&self) -> Rectangle {
        Rectangle::default()
    }
}
