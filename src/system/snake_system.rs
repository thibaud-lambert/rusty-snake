use {ARENA_HEIGHT,ARENA_WIDTH};

use std::time::Duration;
use amethyst::core::timing::Time;
use amethyst::input::InputHandler;
use amethyst::ecs::{Fetch, FetchMut, WriteStorage, System};
use amethyst::core::transform::LocalTransform;

use cgmath::{Vector3,ElementWise};

use snake::{Snake,SnakePart};

#[derive(PartialEq,Clone,Copy)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    BOT
}

pub struct SnakeSystem {
    delta_time : Duration,
    update_rate : Duration,
    curr_dir : Direction,
    target_dir : Direction
}

impl SnakeSystem {
    pub fn new() -> SnakeSystem {
        SnakeSystem {
            delta_time : Duration::new(0,0),
            update_rate : Duration::from_millis(500),
            curr_dir : Direction::UP,
            target_dir : Direction::UP

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

        self.delta_time += time.delta_time();
        if self.delta_time < self.update_rate {
            return;
        }
        self.delta_time -= self.update_rate;

        self.curr_dir = self.target_dir;

        let move_dir = match self.curr_dir {
            Direction::UP => Vector3::new(0.0,1.0,0.0),
            Direction::BOT => Vector3::new(0.0,-1.0,0.0),
            Direction::RIGHT => Vector3::new(1.0,0.0,0.0),
            Direction::LEFT => Vector3::new(-1.0,0.0,0.0),
        };

        // Find next position
        let mut head_pos = transforms.get_mut(snake.head).unwrap().translation;
        head_pos += move_dir;
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
