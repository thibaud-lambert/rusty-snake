use std::time::Duration;
use amethyst::core::timing::Time;
use amethyst::ecs::{Fetch, FetchMut, System};

pub struct Turn(pub bool);

pub struct TurnSystem {
    delta_time : Duration,
    update_rate : Duration,
}

impl TurnSystem {
    pub fn new() -> TurnSystem {
        TurnSystem {
            delta_time : Duration::new(0,0),
            update_rate : Duration::from_millis(200),
        }
    }
}

impl<'a> System<'a> for TurnSystem {
    type SystemData = (
        Fetch<'a, Time>,
        FetchMut<'a, Turn>,
    );

    fn run(&mut self, (time, mut turn): Self::SystemData) {
        self.delta_time += time.delta_time();
        if self.delta_time > self.update_rate {
            self.delta_time -= self.update_rate;
            turn.0 = true;
        } else {
            turn.0 = false;
        }
    }
}
