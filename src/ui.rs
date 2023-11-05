use bevy::prelude::*;

pub const TEXT_COLOR: Color = Color::WHITE;
pub const SCORE_COLOR: Color = Color::WHITE;

pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub const BACKGROUND_COLOR: Color = Color::GOLD;
pub const PADDLE_COLOR: Color = Color::WHITE;
pub const WALL_COLOR: Color = Color::LIME_GREEN;
pub const BALL_COLOR: Color = Color::WHITE;
pub const BRICK_COLOR: Color = Color::WHITE;

#[derive(Resource)]
pub struct ScoreBoard {
    pub score: usize,
}

pub fn update_scoreboard(
    scoreboard: Res<ScoreBoard>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
