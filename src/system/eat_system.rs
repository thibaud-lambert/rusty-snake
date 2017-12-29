use amethyst::ecs::{Entities, Fetch, FetchMut, System, ReadStorage, Join};
use amethyst::core::transform::LocalTransform;

use system::Turn;
use snake::*;

pub struct EatSystem;

impl<'a> System<'a> for EatSystem {
    type SystemData = (
        Fetch<'a, Turn>,
        FetchMut<'a, Snake>,
        Entities<'a>,
        ReadStorage<'a, LocalTransform>,
        ReadStorage<'a, SnakePart>,
        ReadStorage<'a, Food>,
    );

    fn run(&mut self, (turn, mut snake, entities, transforms, parts, foods) : Self::SystemData) {
        if !turn.0 {
            return;
        }

        let head_pos = transforms.get(snake.head).unwrap().translation;

        for (entity, transform, _) in (&*entities, &transforms, &foods).join() {
            if head_pos == transform.translation {
                let _ = entities.delete(entity);
                snake.growing = true;
            }
        }
    }
}
