use {ARENA_HEIGHT,ARENA_WIDTH};

use rand::thread_rng;
use amethyst::ecs::{Entities, Fetch, System, LazyUpdate};
use amethyst::core::transform::{LocalTransform,Transform};
use cgmath::{Vector3,ElementWise};
use rand::Rand;

use system::Turn;
use snake::FoodResource;


pub struct FoodSystem {
    spawn_rate : u32,
    turn_counter : u32
}

impl FoodSystem {
    pub fn new() -> FoodSystem {
        FoodSystem {
            spawn_rate : 10,
            turn_counter : 0
        }
    }
}

impl<'a> System<'a> for FoodSystem {
    type SystemData = (
        Fetch<'a, Turn>,
        Fetch<'a, FoodResource>,
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
    );

    fn run(&mut self, (turn,food_resource,entities,updater) : Self::SystemData) {
        if turn.0 {
            self.turn_counter+=1;
        }
        if self.turn_counter > self.spawn_rate {
            self.turn_counter = 0;

            let e = entities.create();

            let mut rng = thread_rng();
            let mut local_transform = LocalTransform::default();
            local_transform.translation = Vector3::rand(&mut rng).mul_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,0.0));
            local_transform.translation.x = local_transform.translation.x.floor() + 0.5;
            local_transform.translation.y = local_transform.translation.y.floor() + 0.5;

            updater.insert(e,food_resource.mesh.clone());
            updater.insert(e,food_resource.material.clone());
            updater.insert(e,local_transform);
            updater.insert(e,Transform::default());
        }
    }
}
