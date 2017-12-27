use {CELL_SIZE,ARENA_HEIGHT,ARENA_WIDTH,SNAKE_COLOUR};
use amethyst::ecs::{Component, VecStorage};
use amethyst::core::transform::LocalTransform;
use amethyst::core::transform::Transform as transformTransform;
use amethyst::ecs::World;
use cgmath::Vector3;

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
    .with(transformTransform::default())
    .build();
}
