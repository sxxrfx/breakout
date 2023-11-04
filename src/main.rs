use bevy::{
    app::PluginGroupBuilder,
    core_pipeline::clear_color::ClearColorConfig, prelude::*,
    sprite::MaterialMesh2dBundle,
    time::fixed_timestep::FixedUpdateError,
};

pub const SCREEN_HEIGHT: f32 = 800.0;
pub const SCREEN_WIDTH: f32 = 1200.0;

pub const WALL_THICKNESS: f32 = 4.0;

pub const LEFT_WALL: f32 =
    -(SCREEN_WIDTH - WALL_THICKNESS) / 2.0;
pub const RIGHT_WALL: f32 =
    (SCREEN_WIDTH - WALL_THICKNESS) / 2.0;
pub const TOP_WALL: f32 = (SCREEN_HEIGHT - WALL_THICKNESS) / 2.0;
pub const BOTTOM_WALL: f32 =
    -(SCREEN_HEIGHT - WALL_THICKNESS) / 2.0;

pub const PADDLE_SPEED: f32 = 900.0;
pub const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
pub const PADDLE_PADDING: f32 = 0.0;

pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

pub const BALL_RADIUS: f32 = 10.0;
pub const BALL_STARTING_POSITION: Vec3 =
    Vec3::new(0.0, 0.0, 0.0);
pub const BALL_SIZE: Vec3 =
    Vec3::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0, 0.0);
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.0, 0.0);
pub const BALL_SPEED: f32 = 500.0;

pub const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub const BACKGROUND_COLOR: Color = Color::GOLD;
pub const PADDLE_COLOR: Color = Color::WHITE;
pub const WALL_COLOR: Color = Color::LIME_GREEN;
pub const BALL_COLOR: Color = Color::WHITE;
pub const BRICK_COLOR: Color = Color::WHITE;
pub const TEXT_COLOR: Color = Color::WHITE;
pub const SCORE_COLOR: Color = Color::WHITE;

#[derive(Resource)]
pub struct ScoreBoard {
    pub score: usize,
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Brick;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Resource)]
pub struct CollisionSound(Handle<AudioSource>);

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
    fn new(location: WallLocation) -> WallBundle {
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

fn move_paddle(
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
