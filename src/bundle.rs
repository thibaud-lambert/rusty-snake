use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::core::timing::Time;

use snake::{SnakePart,Food};
use system::*;

pub struct SnakeBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SnakeBundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.add_resource(Time::default());
        world.add_resource(Turn(false));
        world.register::<SnakePart>();
        world.register::<Food>();

        Ok(
            builder
            .add(TurnSystem::new(), "turn_system", &[])
            .add(SnakeSystem::new(), "snake_system", &["input_system","turn_system"])
            .add(EatSystem, "eat_system", &["snake_system","turn_system"])
            .add(FoodSystem::new(), "food_system", &["snake_system","turn_system"]),
        )
    }
}
