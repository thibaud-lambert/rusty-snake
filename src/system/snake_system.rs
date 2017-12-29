use {ARENA_HEIGHT,ARENA_WIDTH};

use amethyst::input::InputHandler;
use amethyst::ecs::{Entities, Fetch, FetchMut, WriteStorage, System};
use amethyst::core::transform::{LocalTransform,Transform};
use amethyst::renderer::{Material, MeshHandle};

use cgmath::{Vector3,ElementWise};

use snake::{Snake,SnakePart,SnakeResource};
use system::Turn;

#[derive(PartialEq,Clone,Copy)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    BOT
}

pub struct SnakeSystem {
    curr_dir : Direction,
    target_dir : Direction
}

impl SnakeSystem {
    pub fn new() -> SnakeSystem {
        SnakeSystem {
            curr_dir : Direction::UP,
            target_dir : Direction::UP
        }
    }
}

impl<'a> System<'a> for SnakeSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, SnakePart>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, LocalTransform>,
        WriteStorage<'a, MeshHandle>,
        WriteStorage<'a, Material>,
        Fetch<'a, Turn>,
        Fetch<'a, SnakeResource>,
        FetchMut<'a, Snake>,
        Fetch<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, mut parts, mut transforms, mut ltransforms, mut meshes, mut materials, turn, snake_resource, mut snake, input): Self::SystemData) {

        let vaxis = input.axis_value("vertical_axis");
        if let Some(vmov) = vaxis {
            if vmov as f32 == 1.0 && self.curr_dir != Direction::BOT {
                self.target_dir = Direction::UP;
            } else if vmov as f32 == -1.0 && self.curr_dir != Direction::UP {
                self.target_dir = Direction::BOT;
            }
        }

        let haxis = input.axis_value("horizontal_axis");
        if let Some(hmov) = haxis {
            if hmov as f32 == 1.0 && self.curr_dir != Direction::LEFT {
                self.target_dir = Direction::RIGHT;
            } else if hmov as f32 == -1.0  && self.curr_dir != Direction::RIGHT {
                self.target_dir = Direction::LEFT;
            }
        }

        if !turn.0 {
            return;
        }

        self.curr_dir = self.target_dir;

        let move_dir = match self.curr_dir {
            Direction::UP => Vector3::new(0.0,1.0,0.0),
            Direction::BOT => Vector3::new(0.0,-1.0,0.0),
            Direction::RIGHT => Vector3::new(1.0,0.0,0.0),
            Direction::LEFT => Vector3::new(-1.0,0.0,0.0),
        };

        // Find next position
        let mut head_pos = ltransforms.get_mut(snake.head).unwrap().clone();
        head_pos.translation += move_dir;
        head_pos.translation.add_assign_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,0.0));
        head_pos.translation.rem_assign_element_wise(Vector3::new(ARENA_WIDTH,ARENA_HEIGHT,1.0));

        if snake.growing {
            snake.growing = false;

            let e = entities.create();
            transforms.insert(e,Transform::default());
            parts.insert(e,SnakePart(None));
            meshes.insert(e,snake_resource.mesh.clone());
            materials.insert(e,snake_resource.material.clone());
            ltransforms.insert(e,head_pos);
            parts.get_mut(snake.head).unwrap().0 = Some(e);
            snake.head = e;
        } else {
            // // Tail become head
            parts.get_mut(snake.head).unwrap().0 = Some(snake.tail);
            snake.head = snake.tail;
            snake.tail = parts.get(snake.tail).unwrap().0.unwrap();
            parts.get_mut(snake.head).unwrap().0 = None;
            ltransforms.get_mut(snake.head).unwrap().translation = head_pos.translation;
        }
    }
}
