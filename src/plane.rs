use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Plane {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,

    pub engine_power: f32,
    pub engine_heading: f32,

    pub heading: f32,
    pub speed: f32,

    pub start_position: [f32; 2],

    pub flip: bool,

    pub energy: u32,
    pub dead: bool,

    pub spin: bool,
    pub invincible: bool,
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            radius: 10.0,

            engine_power: 5.0,
            engine_heading: 0.0,

            heading: 0.0,
            speed: 0.0,

            start_position: [0.0, 0.0],

            flip: false,

            energy: 100,

            dead: false,
            spin: false,
            invincible: false,
        }
    }
}
