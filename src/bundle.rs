use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::core::timing::Time;

use snake::SnakePart;
use system::SnakeSystem;

pub struct SnakeBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SnakeBundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.add_resource(Time::default());
        world.register::<SnakePart>();

        Ok(
            builder
            .add(SnakeSystem::new(), "snake_system", &["input_system"]),
        )
    }
}
