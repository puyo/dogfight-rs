use crate::level;
use crate::plane::Plane;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct PlaneSystem;

impl<'s> System<'s> for PlaneSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Plane>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, planes, input): Self::SystemData) {
        for (_plane, transform) in (&planes, &mut transforms).join() {
            let movement = input.axis_value("left");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let paddle_y = transform.translation().y;
                    transform.set_y(
                        (paddle_y + scaled_amount)
                            .min(level::HEIGHT - 10.0 * 0.5)
                            .max(level::HEIGHT * 0.5),
                    );
                    println!("Moving {}", mv_amount);
                }
            }
        }
    }
}
