use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

mod exit_system;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.3, 0.9)))
        .add_startup_system(setup.system())
        .add_system(plane_movement_system.system())
        .add_system(shot_collision_system.system())
        .add_system(shot_movement_system.system())
        .add_system(exit_system::exit_system.system())
        .run();
}

struct Plane {
    velocity: Vec3,
}

struct Shot {
    velocity: Vec3,
}

#[allow(dead_code)]
enum Collider {
    Solid,
    Scorable,
    Plane,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // plane
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .insert(Plane {
            velocity: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Collider::Plane);

    // shot
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Shot {
            velocity: 200.0 * Vec3::new(2.0, 2.0, 0.0).normalize(),
        });

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            // brick
            commands
                .spawn_bundle(SpriteBundle {
                    material: brick_material.clone(),
                    sprite: Sprite::new(brick_size),
                    transform: Transform::from_translation(brick_position),
                    ..Default::default()
                })
                .insert(Collider::Scorable);
        }
    }
}

fn plane_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Plane, &mut Transform)>,
    mut windows: ResMut<Windows>,
) {
    if let Ok((mut plane, mut transform)) = query.single_mut() {
        let delta_seconds = f32::min(0.2, time.delta_seconds());

        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            plane.velocity.x -= 10.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            plane.velocity.x += 10.0;
        }

        let translation = &mut transform.translation;

        // move the plane horizontally
        *translation += plane.velocity * delta_seconds;

        let window = windows.get_primary_mut().unwrap();
        let w2 = window.width() / 2.0;
        let h2 = window.height() / 2.0;

        if translation.x < w2 {
            translation.x += window.width();
        }
        if translation.x >= w2 {
            translation.x -= window.width();
        }
        if translation.y < h2 {
            translation.y += window.height();
        }
        if translation.y >= h2 {
            translation.y -= window.height();
        }
    }
}

fn shot_movement_system(
    time: Res<Time>,
    mut shot_query: Query<(&Shot, &mut Transform)>,
    mut windows: ResMut<Windows>,
) {
    if let Ok((shot, mut transform)) = shot_query.single_mut() {
        // clamp the timestep to stop the shot from escaping when the game starts
        let delta_seconds = f32::min(0.2, time.delta_seconds());

        let translation = &mut transform.translation;

        *translation += shot.velocity * delta_seconds;

        let window = windows.get_primary_mut().unwrap();
        let w2 = window.width() / 2.0;
        let h2 = window.height() / 2.0;

        if translation.x < w2 {
            translation.x += window.width();
        }
        if translation.x >= w2 {
            translation.x -= window.width();
        }
        if translation.y < h2 {
            translation.y += window.height();
        }
        if translation.y >= h2 {
            translation.y -= window.height();
        }
    }
}

fn shot_collision_system(
    mut commands: Commands,
    mut shot_query: Query<(&mut Shot, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    if let Ok((mut shot, shot_transform, sprite)) = shot_query.single_mut() {
        let shot_size = sprite.size;
        let velocity = &mut shot.velocity;

        // check collision with walls
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                shot_transform.translation,
                shot_size,
                transform.translation,
                sprite.size,
            );

            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    commands.entity(collider_entity).despawn();
                }

                // reflect the shot when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the shot's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is
                // also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}
