use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

pub struct DogfightBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for DogfightBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        Ok(())
    }
}
