use {ARENA_HEIGHT,ARENA_WIDTH,SNAKE_COLOUR};

use amethyst::ecs::{Entity, Component, VecStorage, World};
use amethyst::core::transform::{LocalTransform,Transform};
use cgmath::Vector3;
use rendering::*;

pub struct Snake {
    pub tail : Entity,
    pub head : Entity,
    pub growing : bool
}

pub struct SnakePart(pub Option<Entity>);

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

    transform.translation.y -= 1.0;

    let mid = world.create_entity()
    .with(SnakePart(Some(head)))
    .with(mesh.clone())
    .with(material.clone())
    .with(transform.clone())
    .with(Transform::default())
    .build();

    transform.translation.y -= 1.0;

    let tail = world.create_entity()
    .with(SnakePart(Some(mid)))
    .with(mesh)
    .with(material)
    .with(transform)
    .with(Transform::default())
    .build();

    world.add_resource(Snake {head:head,tail:tail,growing:false});
}
