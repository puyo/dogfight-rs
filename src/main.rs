use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    render::pass::ClearColor,
    render::renderer::*,
    sprite::collide_aabb::{collide, Collision},
};
use std::time::Duration;

mod exit_system;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.3, 0.9)))
        .add_startup_system(setup.system())
        .add_system(shot_death_system.system())
        .add_system(plane_input_system.system())
        .add_system(shot_collision_system.system())
        .add_system(movement_system.system())
        .add_system(exit_system::exit_system.system())
        .run();
}

#[derive(Debug)]
struct Mob {
    thrust_angle: f32,
    thrust_power: f32,
    thrust_power_max: f32,
    thrust_power_min: f32,
    thrust_power_speed: f32,
    thrust_turn_speed: f32,
    max_speed: f32,
    lift_factor: f32,
    gravity_factor: f32,
    drag_factor: f32,
    velocity: Vec3,
}

struct Plane {}
struct Shot {}

#[allow(dead_code)]
enum Collider {
    Solid,
    Scorable,
    Plane,
}

const GRAVITY: f32 = 2.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // textures
    let texture_handle = asset_server.load("textures/plane.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(12.0, 11.0),
        10,
        1,
        Vec2::new(1.0, 1.0),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // planes
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform {
                translation: Vec3::new(0.0, -215.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(4.0, 4.0, 0.0),
            },
            ..Default::default()
        })
        .insert(Plane {})
        .insert(Mob {
            thrust_angle: 0.0,
            thrust_power: 10.0,
            thrust_power_max: 10.0,
            thrust_power_min: 0.0,
            thrust_power_speed: 0.2,
            thrust_turn_speed: 0.05,
            max_speed: 200.0,
            lift_factor: 0.01,
            drag_factor: 0.01,
            gravity_factor: 1.0,
            velocity: Vec3::new(140.0, 0.0, 0.0),
        })
        .insert(Collider::Plane)
        .insert(Timer::from_seconds(0.0, false));

    // shot
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Shot {})
        .insert(Mob {
            thrust_angle: 0.0,
            thrust_power: 0.0,
            thrust_power_speed: 0.0,
            thrust_power_max: 0.0,
            thrust_power_min: 0.0,
            thrust_turn_speed: 0.0,
            max_speed: 300.0,
            lift_factor: 0.0,
            drag_factor: 0.0,
            gravity_factor: 0.0,
            velocity: Vec3::new(140.0, 140.0, 0.0),
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

fn plane_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Plane, &mut Mob, &Transform, &mut Timer)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_plane, mut mob, transform, mut timer) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            mob.thrust_angle += mob.thrust_turn_speed;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            mob.thrust_angle -= mob.thrust_turn_speed;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            mob.thrust_power += mob.thrust_power_speed;
            if mob.thrust_power > mob.thrust_power_max {
                mob.thrust_power = mob.thrust_power_max;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            mob.thrust_power -= mob.thrust_power_speed;
            if mob.thrust_power < mob.thrust_power_min {
                mob.thrust_power = mob.thrust_power_min;
            }
        }

        timer.tick(time.delta());
        if timer.finished() && keyboard_input.pressed(KeyCode::Space) {
            timer.set_duration(Duration::from_secs_f32(0.5));
            timer.reset();

            // shot
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                    transform: Transform {
                        translation: transform.translation
                            + transform.rotation.mul_vec3(Vec3::new(25.0, 0.0, 0.0)),
                        rotation: transform.rotation,
                        scale: Vec3::new(1.0, 1.0, 0.0),
                    },
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                })
                .insert(Shot {})
                .insert(Mob {
                    thrust_angle: mob.thrust_angle,
                    thrust_power: 0.0,
                    thrust_power_speed: 0.0,
                    thrust_power_max: 0.0,
                    thrust_power_min: 0.0,
                    thrust_turn_speed: 0.0,
                    max_speed: 400.0,
                    lift_factor: 0.0,
                    drag_factor: 0.0,
                    gravity_factor: 0.0,
                    velocity: mob.velocity
                        + transform.rotation.mul_vec3(Vec3::new(100.0, 0.0, 0.0)),
                })
                .insert(Timer::from_seconds(2.0, false));
        }
    }
}

fn shot_death_system(
    time: Res<Time>,
    mut query: Query<(Entity, &Shot, &mut Timer)>,
    mut commands: Commands,
) {
    for (entity, _shot, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Mob, &mut Transform)>,
    mut windows: ResMut<Windows>,
) {
    for (mut mob, mut transform) in query.iter_mut() {
        let delta_seconds = f32::min(0.2, time.delta_seconds());

        // thrust alters velocity
        mob.velocity.x += mob.thrust_power * mob.thrust_angle.cos();
        mob.velocity.y += mob.thrust_power * mob.thrust_angle.sin();

        let speed: f32 = mob.velocity.length();

        let vel_norm: Vec3 = mob.velocity.normalize();

        // wind resistance
        let mut new_speed: f32 = speed * (1.0 - mob.drag_factor);

        // maximum speed
        if new_speed > mob.max_speed {
            new_speed = mob.max_speed;
        }
        mob.velocity = new_speed * vel_norm;

        // dot product with a vector going up
        let up: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let upness: f32 = up.dot(mob.velocity).abs();
        mob.velocity.y += upness * mob.lift_factor - (GRAVITY * mob.gravity_factor);

        // if mob.thrust_power_speed > 0.0 {
        //     println!("upness = {}", upness);
        // }

        transform.rotation = Quat::from_rotation_z(mob.thrust_angle);
        transform.translation += mob.velocity * delta_seconds;

        let window = windows.get_primary_mut().unwrap();
        let w2 = window.width() / 2.0;
        let h2 = window.height() / 2.0;

        if transform.translation.x < w2 {
            transform.translation.x += window.width();
        }
        if transform.translation.x >= w2 {
            transform.translation.x -= window.width();
        }
        if transform.translation.y < h2 {
            transform.translation.y += window.height();
        }
        if transform.translation.y >= h2 {
            transform.translation.y -= window.height();
        }
    }
}

fn shot_collision_system(
    mut commands: Commands,
    mut shot_query: Query<(&mut Shot, &mut Mob, &Transform, &Sprite)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    collider_sprite_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    collider_tex_query: Query<(
        Entity,
        &Collider,
        &Transform,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    // loop through shots
    for (_shot, mut mob, shot_transform, sprite) in shot_query.iter_mut() {
        let shot_size = sprite.size;
        let shot_velocity = &mut mob.velocity;

        // check collision with all things that have a (Collider, Transform, TextureAtlasSprite)
        for (entity, collider, transform, sprite, tex_handle) in collider_tex_query.iter() {
            let tex_atlas = texture_atlases.get(tex_handle).unwrap();
            let tex = tex_atlas.textures[0];

            if let Some(handle) = RenderResource::texture(sprite) {
                println!("{:?}", handle);
            }

            let collision = collide(
                shot_transform.translation,
                shot_size,
                transform.translation,
                tex.max,
            );

            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    commands.entity(entity).despawn();
                }

                // reflect the shot when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the shot's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = shot_velocity.x > 0.0,
                    Collision::Right => reflect_x = shot_velocity.x < 0.0,
                    Collision::Top => reflect_y = shot_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = shot_velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    shot_velocity.x = -shot_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    shot_velocity.y = -shot_velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is
                // also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }

        // check collision with all things that have a (Collider, Transform, Sprite)
        for (collider_entity, collider, transform, sprite) in collider_sprite_query.iter() {
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
                    Collision::Left => reflect_x = shot_velocity.x > 0.0,
                    Collision::Right => reflect_x = shot_velocity.x < 0.0,
                    Collision::Top => reflect_y = shot_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = shot_velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    shot_velocity.x = -shot_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    shot_velocity.y = -shot_velocity.y;
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
