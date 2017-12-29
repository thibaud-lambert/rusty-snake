extern crate amethyst;
extern crate cgmath;
extern crate rand;

mod snake;
mod rendering;
mod bundle;
mod system;

use std::time::Duration;
use amethyst::prelude::*;
use amethyst::ecs::World;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput, Pipeline, RenderBundle, RenderSystem, Stage, VirtualKeyCode, WindowEvent, PosTex};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;

use rendering::*;
use snake::*;
use bundle::*;

const ARENA_HEIGHT: f32 = 20.0;
const ARENA_WIDTH: f32 = 20.0;
const SNAKE_RADIUS: f32 = 0.4;
const SNAKE_COLOUR: [f32; 4] = [0.2, 0.3, 0.8, 1.0];
const FOOD_RADIUS: f32 = 0.3;
const FOOD_COLOUR: [f32; 4] = [0.4, 0.8, 0.3, 1.0];

struct SnakeGame;

impl State for SnakeGame {
    fn on_start(&mut self, world: &mut World) {
        initialise_camera(world);
        initialise_snake(world);
        initialise_food(world);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}


fn run() -> Result<(), amethyst::Error> {
    let path = format!(
        "{}/resources/display.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let key_bindings_path = format!(
        "{}/resources/input.ron",
        env!("CARGO_MANIFEST_DIR")
);

    let game = Application::build("./", SnakeGame)?
    .with_frame_limit(
        FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
        144,
    )
    .with_bundle(
           InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path),)?
    .with_bundle(RenderBundle::new())?
    .with_bundle(TransformBundle::new())?
    .with_bundle(SnakeBundle)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
        .clear_target([0.25, 0.25, 0.25, 1.0], 1.0)
        .with_pass(DrawFlat::<PosTex>::new())
    );

    Ok(
        game.with_local(RenderSystem::build(pipe, Some(config))?)
        .build()?
        .run(),
    )
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
