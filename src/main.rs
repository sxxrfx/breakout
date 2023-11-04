use bevy::{
    app::PluginGroupBuilder, prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::{
    ball::{
        Ball, BALL_SIZE, BALL_SPEED, BALL_STARTING_POSITION,
        INITIAL_BALL_DIRECTION,
    },
    paddle::{
        move_paddle, Paddle, GAP_BETWEEN_PADDLE_AND_FLOOR,
        PADDLE_SIZE,
    },
    wall::{WallBundle, WallLocation, BOTTOM_WALL}, SCREEN_WIDTH, SCREEN_HEIGHT,
};
use physics::{
    Collider, CollisionEvent, CollisionSound, Velocity,
};
use ui::{
    ScoreBoard, BACKGROUND_COLOR, BALL_COLOR, PADDLE_COLOR,
};

mod components;

mod physics;
mod ui;

fn main() {
    App::new()
        .add_plugins(custom_plugins())
        .insert_resource(ScoreBoard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc,))
        .add_systems(FixedUpdate, (move_paddle,))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound =
        asset_server.load("sounds/breakout_collision.ogg");
    commands
        .insert_resource(CollisionSound(ball_collision_sound));

    // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle,
        Collider,
    ));

    // Wall
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::default().into())
                .into(),
            material: materials
                .add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(
                BALL_STARTING_POSITION,
            )
            .with_scale(BALL_SIZE),
            ..Default::default()
        },
        Ball,
        Velocity(
            INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED,
        ),
    ));
}

fn custom_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(ImagePlugin::default_nearest()).set(
        WindowPlugin {
            primary_window: Some(Window {
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                title: "App".into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        },
    )
}
