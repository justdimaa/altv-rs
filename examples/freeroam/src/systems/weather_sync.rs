use altv::ecs::{Join, System, WriteStorage};
use altv::sdk::elements::*;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct WeatherSync {
    last_sync: Instant,
    last_weather_update: Instant,
    current_weather: u32,
}

impl WeatherSync {
    pub fn new() -> Self {
        WeatherSync {
            last_sync: Instant::now(),
            last_weather_update: Instant::now(),
            current_weather: 0,
        }
    }
}

impl<'a> System<'a> for WeatherSync {
    type SystemData = WriteStorage<'a, CPlayer>;

    fn run(&mut self, mut cplayers: Self::SystemData) {
        if self.last_weather_update.elapsed() >= Duration::from_secs(1800) {
            let mut rng = rand::thread_rng();
            self.current_weather = rng.gen_range(0, 10);
            self.last_weather_update = Instant::now();

            altv::sdk::log::info(&format!(
                "[WeatherSync] Change weather to {}",
                self.current_weather
            ));
        }

        if self.last_sync.elapsed() >= Duration::from_secs(1) {
            for cplayer in (&mut cplayers).join() {
                cplayer.set_date_time(1, 1, 1970, 12, 0, 0);
                cplayer.set_weather(self.current_weather);
            }

            self.last_sync = Instant::now();
        }
    }
}
