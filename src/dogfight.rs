use crate::plane::Plane;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{
        Camera, Event, KeyboardInput, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, VirtualKeyCode,
        WindowEvent,
    },
};

pub struct Dogfight;

impl SimpleState for Dogfight {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let plane_sprite_sheet = load_plane_sprite_sheet(world);

        world.register::<Plane>();

        // Setup our game.
        init_planes(world, plane_sprite_sheet.clone());
        init_camera(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> amethyst::Trans<amethyst::GameData<'static, 'static>, amethyst::StateEvent> {
        match event {
            StateEvent::Window(Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            }) => Trans::Quit,
            StateEvent::Window(Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }) => Trans::Quit,
            _ => Trans::None,
        }
    }
}

fn load_plane_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let plane_texture = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/plane.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "texture/plane.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        plane_texture, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            crate::level::WIDTH,
            0.0,
            crate::level::HEIGHT,
        )))
        .with(transform)
        .build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn init_planes(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let x = crate::level::WIDTH / 2.0;
    let y = crate::level::HEIGHT / 2.0;

    let p1 = Plane {
        velocity: [10.0, 10.0],
        radius: 10.0,
        engine_power: 10.0,
        engine_heading: 0.0,
        heading: 0.0,
        speed: 0.0,
        start_position: [x - 10.0, y - 10.0],
        flip: false,
        energy: 100,
        dead: false,
        spin: false,
        invincible: false,
    };

    let p2 = Plane {
        velocity: [10.0, 10.0],
        radius: 10.0,
        engine_power: 10.0,
        engine_heading: 0.0,
        heading: 0.0,
        speed: 0.0,
        start_position: [x + 10.0, y + 10.0],
        flip: false,
        energy: 100,
        dead: false,
        spin: false,
        invincible: false,
    };

    init_plane(p1, world, sprite_sheet.clone());
    init_plane(p2, world, sprite_sheet.clone());
}

fn init_plane(plane: Plane, world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut t = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    t.set_xyz(plane.start_position[0], plane.start_position[1], 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(plane)
        .with(t)
        .build();
}
