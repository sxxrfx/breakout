use bevy::prelude::*;

pub const BRICK_SIZE: Vec2 = Vec2::new(160.0, 50.0);
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 400.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 50.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 50.0;

#[derive(Component)]
pub struct Brick;
