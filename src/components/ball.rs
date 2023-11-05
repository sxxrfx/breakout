use bevy::prelude::*;

pub const BALL_RADIUS: f32 = 15.0;
pub const BALL_STARTING_POSITION: Vec3 =
    Vec3::new(0.0, -50.0, 1.0);
pub const BALL_SIZE: Vec3 =
    Vec3::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0, 0.0);
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
pub const BALL_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Ball;
