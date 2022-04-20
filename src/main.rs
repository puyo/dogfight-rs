use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use std::cmp;
use std::f32::consts::PI;
use std::time::Duration;

mod exit_system;

#[derive(Debug, Component)]
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

#[derive(Component)]
struct Plane {
    steer_left_key: KeyCode,
    steer_right_key: KeyCode,
    throttle_up_key: KeyCode,
    throttle_down_key: KeyCode,
    fire_gun_key: KeyCode,
    cannot_shoot_timer: Timer,
    cannot_flip_timer: Timer,
    flip_animation_timer: Timer,
    flipping: bool,
}

#[derive(Component)]
struct Shot {}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Expiry {
    timer: Timer,
}

const GRAVITY: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.3, 0.9)))
        .add_startup_system(setup)
        .add_system(shot_expiry_system)
        .add_system(plane_input_system)
        .add_system(shot_collision_system)
        .add_system(movement_system)
        .add_system(plane_flip_system)
        .add_system(exit_system::exit_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // textures
    let texture_handle1 = asset_server.load("textures/plane.png");
    let texture_handle2 = asset_server.load("textures/plane-green.png");

    let texture_atlas1 =
        TextureAtlas::from_grid_with_padding(texture_handle1, Vec2::new(12.0, 11.0), 10, 1, Vec2::new(1.0, 1.0));

    let texture_atlas2 =
        TextureAtlas::from_grid_with_padding(texture_handle2, Vec2::new(12.0, 11.0), 10, 1, Vec2::new(1.0, 1.0));
    let texture_atlas_handle1 = texture_atlases.add(texture_atlas1);
    let texture_atlas_handle2 = texture_atlases.add(texture_atlas2);

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // planes
    commands
        .spawn()
        .insert(Plane {
            steer_left_key: KeyCode::Left,
            steer_right_key: KeyCode::Right,
            throttle_up_key: KeyCode::Up,
            throttle_down_key: KeyCode::Down,
            fire_gun_key: KeyCode::Space,
            cannot_shoot_timer: Timer::from_seconds(0.0, false),
            cannot_flip_timer: Timer::from_seconds(0.0, false),
            flip_animation_timer: Timer::from_seconds(0.1, true),
            flipping: false,
        })
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
        .insert(Collider)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle1,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform {
                translation: Vec3::new(-100.0, -215.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(4.0, 4.0, 0.0),
            },
            ..default()
        });

    commands
        .spawn()
        .insert(Plane {
            steer_left_key: KeyCode::A,
            steer_right_key: KeyCode::D,
            throttle_up_key: KeyCode::W,
            throttle_down_key: KeyCode::S,
            fire_gun_key: KeyCode::E,
            cannot_shoot_timer: Timer::from_seconds(0.0, false),
            cannot_flip_timer: Timer::from_seconds(0.0, false),
            flip_animation_timer: Timer::from_seconds(0.05, true),
            flipping: false,
        })
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
        .insert(Collider)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle2,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, 0.0),
                rotation: Quat::from_rotation_z(3.14),
                scale: Vec3::new(4.0, 4.0, 0.0),
            },
            ..default()
        });
}

fn plane_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Plane, &mut Mob, &Transform)>,
    mut commands: Commands,
) {
    let mut turned: bool = false;

    for (mut plane, mut mob, transform) in query.iter_mut() {
        plane.cannot_shoot_timer.tick(time.delta());

        if keyboard_input.pressed(plane.steer_left_key) {
            mob.thrust_angle += mob.thrust_turn_speed;
            turned = true;
        }
        if keyboard_input.pressed(plane.steer_right_key) {
            mob.thrust_angle -= mob.thrust_turn_speed;
            turned = true;
        }
        if keyboard_input.pressed(plane.throttle_up_key) {
            mob.thrust_power += mob.thrust_power_speed;
            if mob.thrust_power > mob.thrust_power_max {
                mob.thrust_power = mob.thrust_power_max;
            }
        }
        if keyboard_input.pressed(plane.throttle_down_key) {
            mob.thrust_power -= mob.thrust_power_speed;
            if mob.thrust_power < mob.thrust_power_min {
                mob.thrust_power = mob.thrust_power_min;
            }
        }

        if turned && !plane.flipping {
            plane.cannot_flip_timer.set_duration(Duration::from_secs_f32(0.1));
            plane.cannot_flip_timer.reset();
        }

        if plane.cannot_shoot_timer.finished() && keyboard_input.pressed(plane.fire_gun_key) {
            plane.cannot_shoot_timer.set_duration(Duration::from_secs_f32(0.5));
            plane.cannot_shoot_timer.reset();

            // shot
            commands
                .spawn()
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
                    velocity: mob.velocity + transform.rotation.mul_vec3(Vec3::new(100.0, 0.0, 0.0)),
                })
                .insert(Expiry {
                    timer: Timer::from_seconds(2.0, false),
                })
                .insert_bundle(SpriteBundle {
                    transform: Transform {
                        translation: transform.translation + transform.rotation.mul_vec3(Vec3::new(25.0, 0.0, 0.0)),
                        rotation: transform.rotation,
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        color: Color::rgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                    ..default()
                });
        }
    }
}

fn shot_expiry_system(time: Res<Time>, mut query: Query<(Entity, &mut Expiry)>, mut commands: Commands) {
    for (entity, mut expiry) in query.iter_mut() {
        expiry.timer.tick(time.delta());

        if expiry.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

const PI_HALF: f32 = PI * 0.5;
const PI_ONE_AND_A_HALF: f32 = PI * 1.5;
const PLANE_SPRITE_INDEX_RIGHT_WAY_UP: usize = 1;
const PLANE_SPRITE_INDEX_UPSIDE_DOWN: usize = 9;

fn plane_flip_system(time: Res<Time>, mut query: Query<(&mut Plane, &Transform, &mut TextureAtlasSprite)>) {
    for (mut plane, transform, mut sprite) in query.iter_mut() {
        plane.cannot_flip_timer.tick(time.delta());
        plane.flip_animation_timer.tick(time.delta());

        if plane.cannot_flip_timer.finished() && plane.flip_animation_timer.finished() {
            let (_axis, angle) = transform.rotation.to_axis_angle();
            let sprite_index: usize;

            if PI_HALF < angle && angle < PI_ONE_AND_A_HALF {
                sprite_index = cmp::min(sprite.index + 1, PLANE_SPRITE_INDEX_UPSIDE_DOWN);
            } else {
                sprite_index = cmp::max(sprite.index - 1, PLANE_SPRITE_INDEX_RIGHT_WAY_UP);
            }

            sprite.index = sprite_index;
            plane.flipping =
                PLANE_SPRITE_INDEX_RIGHT_WAY_UP < sprite.index && sprite.index < PLANE_SPRITE_INDEX_UPSIDE_DOWN;
        }
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Mob, &mut Transform)>, mut windows: ResMut<Windows>) {
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
    mut shot_query: Query<(&mut Mob, &Transform, &Sprite), With<Shot>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    collider_sprite_query: Query<&Transform, With<Collider>>,
    collider_tex_query: Query<(&Transform, &Handle<TextureAtlas>), With<Collider>>,
) {
    // loop through shots
    for (mut mob, shot_transform, sprite) in shot_query.iter_mut() {
        if let Some(shot_size) = sprite.custom_size {
            let shot_velocity = &mut mob.velocity;

            // check collision with all things that have a (Collider, Transform, TextureAtlasSprite)
            for (transform, tex_handle) in collider_tex_query.iter() {
                let tex_atlas = texture_atlases.get(tex_handle).unwrap();
                let tex = tex_atlas.textures[0];

                let collision = collide(shot_transform.translation, shot_size, transform.translation, tex.max);

                if let Some(collision) = collision {
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
                        Collision::Inside => { /* nothing */ }
                    }

                    // reflect velocity on the x-axis if we hit something on the x-axis
                    if reflect_x {
                        shot_velocity.x = -shot_velocity.x;
                    }

                    // reflect velocity on the y-axis if we hit something on the y-axis
                    if reflect_y {
                        shot_velocity.y = -shot_velocity.y;
                    }
                }
            }

            // check collision with all things that have a (Collider, Transform, Sprite)
            for transform in collider_sprite_query.iter() {
                let collision = collide(
                    shot_transform.translation,
                    shot_size,
                    transform.translation,
                    transform.scale.truncate(),
                );

                if let Some(collision) = collision {
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
                        Collision::Inside => { /* nothing */ }
                    }

                    // reflect velocity on the x-axis if we hit something on the x-axis
                    if reflect_x {
                        shot_velocity.x = -shot_velocity.x;
                    }

                    // reflect velocity on the y-axis if we hit something on the y-axis
                    if reflect_y {
                        shot_velocity.y = -shot_velocity.y;
                    }
                }
            }
        }
    }
}
