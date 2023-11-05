use bevy::prelude::*;

use crate::{physics::Collider, ui::WALL_COLOR};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH, WALL_THICKNESS};

pub const LEFT_WALL: f32 = -(SCREEN_WIDTH) / 2.0;
pub const RIGHT_WALL: f32 = (SCREEN_WIDTH) / 2.0;
pub const TOP_WALL: f32 = (SCREEN_HEIGHT) / 2.0;
pub const BOTTOM_WALL: f32 = -(SCREEN_HEIGHT) / 2.0;

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_width > 0.0);
        assert!(arena_height > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(
                    WALL_THICKNESS,
                    arena_height + WALL_THICKNESS,
                )
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(
                    arena_width + WALL_THICKNESS,
                    WALL_THICKNESS,
                )
            }
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
        }
    }
}
