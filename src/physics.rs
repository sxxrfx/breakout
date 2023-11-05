use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    components::{ball::Ball, brick::Brick},
    ui::ScoreBoard,
};

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Collider;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x +=
            velocity.x * time_step.period.as_secs_f32();
        transform.translation.y +=
            velocity.y * time_step.period.as_secs_f32();
    }
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<ScoreBoard>,
    mut ball_query: Query<
        (&mut Velocity, &Transform),
        With<Ball>,
    >,
    collider_query: Query<
        (Entity, &Transform, Option<&Brick>),
        With<Collider>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) =
        ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform, maybe_brick) in
        &collider_query
    {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_brick.is_some() {
                scoreboard.score += 1;
                commands.entity(collider_entity).despawn();
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => {
                    reflect_x = ball_velocity.x > 0.0
                }
                Collision::Right => {
                    reflect_x = ball_velocity.x < 0.0
                }
                Collision::Top => {
                    reflect_y = ball_velocity.y < 0.0
                }
                Collision::Bottom => {
                    reflect_y = ball_velocity.y > 0.0
                }
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

pub fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
