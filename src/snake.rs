use {CELL_SIZE,ARENA_HEIGHT,ARENA_WIDTH,SNAKE_COLOUR};

use std::time::Duration;
use amethyst::ecs::{Fetch, Join, System, WriteStorage, ReadStorage, DispatcherBuilder, Component, VecStorage, World};
use amethyst::core::transform::{LocalTransform,Transform};
use amethyst::core::timing::Time;
use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::input::InputHandler;
use cgmath::{Vector3,ElementWise};


use rendering::*;

pub enum SnakePart {
    HEAD
}

impl Component for SnakePart {
    type Storage = VecStorage<SnakePart>;
}


pub fn initialise_snake(world: &mut World) {
    let mut transform = LocalTransform::default();
    transform.translation = Vector3::new(ARENA_WIDTH/2.,ARENA_HEIGHT/2.,0.0);

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, CELL_SIZE, CELL_SIZE),
    );

    let material = create_colour_material(world, SNAKE_COLOUR);

    world.create_entity()
    .with(SnakePart::HEAD)
    .with(mesh)
    .with(material)
    .with(transform)
    .with(Transform::default())
    .build();
}

pub struct SnakeSystem {
    delta_time : Duration,
    update_rate : Duration,
    move_dir : Vector3<f32>,
}

impl SnakeSystem {
    pub fn new() -> SnakeSystem {
        SnakeSystem {
            delta_time : Duration::new(0,0),
            update_rate : Duration::from_millis(500),
            move_dir : Vector3::new(0.0,1.0,0.0)

        }
    }
}

impl<'a> System<'a> for SnakeSystem {
    type SystemData = (
        ReadStorage<'a, SnakePart>,
        WriteStorage<'a, LocalTransform>,
        Fetch<'a, Time>,
        Fetch<'a, InputHandler<String, String>>,
    );


    fn run(&mut self, (parts, mut transforms, time, input): Self::SystemData) {
        let vaxis = input.axis_value("vertical_axis");
        if let Some(vmov) = vaxis {
            if vmov as f32 == 1.0 {
                self.move_dir = Vector3::new(0.0,1.0,0.0);
            } else if vmov as f32 == -1.0 {
                self.move_dir = Vector3::new(0.0,-1.0,0.0);
            }
        }
        let haxis = input.axis_value("horizontal_axis");
        if let Some(hmov) = haxis {
            if hmov as f32 == 1.0 {
                self.move_dir = Vector3::new(1.0,0.0,0.0);
            } else if hmov as f32 == -1.0 {
                self.move_dir = Vector3::new(-1.0,0.0,0.0);
            }
        }

        self.delta_time += time.delta_time();
        if self.delta_time < self.update_rate {
            return;
        }
        self.delta_time -= self.update_rate;

        // Iterate over all parts and move them according to the input the user provided.
        for (part, transform) in (&parts, &mut transforms).join() {
            match part {
                &SnakePart::HEAD => {
                    transform.translation += self.move_dir * CELL_SIZE;
                    transform.translation.add_assign_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,0.0));
                    transform.translation.rem_assign_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,1.0));
                },
            }
        }
    }
}

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
