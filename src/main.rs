use bevy::{
    app::PluginGroupBuilder,
    core_pipeline::clear_color::ClearColorConfig, prelude::*,
};

fn main() {
    App::new()
        .add_plugins(custom_plugins())
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn custom_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(ImagePlugin::default_nearest()).set(
        WindowPlugin {
            primary_window: Some(Window {
                resolution: (640.0, 460.0).into(),
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
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GOLD),
        },
        ..Default::default()
    });

    let texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        texture,
        ..Default::default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const VELOCITY: f32 = 300.0;
    for (mut transform, sprite) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y +=
                VELOCITY * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -=
                VELOCITY * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x +=
                VELOCITY * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -=
                VELOCITY * time.delta_seconds();
        }
    }
}
