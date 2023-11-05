use bevy::prelude::*;

use super::{wall::{LEFT_WALL, RIGHT_WALL}, WALL_THICKNESS};

pub const PADDLE_SPEED: f32 = 900.0;
pub const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
pub const PADDLE_PADDING: f32 = 0.0;
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;


#[derive(Component)]
pub struct Paddle;

pub fn move_paddle(
    mut query: Query<&mut Transform, With<Paddle>>,
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x
        + direction
            * PADDLE_SPEED
            * time_step.period.as_secs_f32();

    let left_bound = LEFT_WALL
        + WALL_THICKNESS
        + PADDLE_SIZE.x / 2.0
        + PADDLE_PADDING;
    let right_bound = RIGHT_WALL
        - WALL_THICKNESS
        - PADDLE_SIZE.x / 2.0
        - PADDLE_PADDING;

    paddle_transform.translation.x =
        new_paddle_position.clamp(left_bound, right_bound);
}
