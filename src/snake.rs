use {ARENA_HEIGHT,ARENA_WIDTH,SNAKE_COLOUR};

use std::time::Duration;
use amethyst::ecs::{Entity, Fetch, FetchMut, System, WriteStorage, DispatcherBuilder, Component, VecStorage, World};
use amethyst::core::transform::{LocalTransform,Transform};
use amethyst::core::timing::Time;
use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::input::InputHandler;
use cgmath::{Vector3,ElementWise};

use rendering::*;

pub struct Snake {
    head : Entity,
    tail : Entity,
    growing : bool
}

pub struct SnakePart(Option<Entity>);

impl Component for SnakePart {
    type Storage = VecStorage<SnakePart>;
}

pub fn initialise_snake(world: &mut World) {
    let mut transform = LocalTransform::default();
    transform.translation = Vector3::new(ARENA_WIDTH/2.,ARENA_HEIGHT/2.,0.0);

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, 1.0, 1.0),
    );

    let material = create_colour_material(world, SNAKE_COLOUR);

    let head = world.create_entity()
    .with(SnakePart(None))
    .with(mesh.clone())
    .with(material.clone())
    .with(transform.clone())
    .with(Transform::default())
    .build();

    let mid = world.create_entity()
    .with(SnakePart(Some(head)))
    .with(mesh.clone())
    .with(material.clone())
    .with(transform.clone())
    .with(Transform::default())
    .build();

    let tail = world.create_entity()
    .with(SnakePart(Some(mid)))
    .with(mesh)
    .with(material)
    .with(transform)
    .with(Transform::default())
    .build();

    world.add_resource(Snake {head:head,tail:tail,growing:false});
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
        WriteStorage<'a, SnakePart>,
        WriteStorage<'a, LocalTransform>,
        Fetch<'a, Time>,
        FetchMut<'a, Snake>,
        Fetch<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut parts, mut transforms, time, mut snake, input): Self::SystemData) {

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

        // Find next position
        let mut head_pos = transforms.get_mut(snake.head).unwrap().translation;
        head_pos += self.move_dir;
        head_pos.add_assign_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,0.0));
        head_pos.rem_assign_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,1.0));


        if snake.growing {
        } else {
            // // Tail become head
            parts.get_mut(snake.head).unwrap().0 = Some(snake.tail);
            snake.head = snake.tail;
            snake.tail = parts.get(snake.tail).unwrap().0.unwrap();
            parts.get_mut(snake.head).unwrap().0 = None;
        }

        transforms.get_mut(snake.head).unwrap().translation = head_pos;
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
