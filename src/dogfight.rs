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
        let sprite_sheet = load_sprite_sheet(world);

        world.register::<Plane>();

        // Setup our game.
        initialise_planes(world, sprite_sheet.clone());
        initialise_camera(world);
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

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture` is a cloneable reference to the texture
    let texture = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
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
fn initialise_planes(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let x = crate::level::WIDTH / 2.0;
    let y = crate::level::HEIGHT / 2.0;

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    left_transform.set_xyz(x - 10.0, y, 0.0);
    right_transform.set_xyz(x + 10.0, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Plane {
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
        })
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Plane {
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
        })
        .with(right_transform)
        .build();
}
