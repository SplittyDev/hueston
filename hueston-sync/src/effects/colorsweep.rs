use hueston::{HueBridgeClient, HueLight, HueLightBatch};
use rand::prelude::*;
use std::time::Duration;

pub struct ColorsweepEffect {}

impl ColorsweepEffect {
    pub fn run(&self, client: &HueBridgeClient, lights: Vec<HueLight>) {
        let mut rng = thread_rng();

        // Pick a random hue to start with
        let mut hue: u16 = rng.gen_range(0, std::u16::MAX / 100);

        // Turn all lights on
        for (i, _) in lights.iter().filter(|light| !light.is_on()).enumerate() {
            let (light_i, params) = {
                let mut batch = HueLightBatch::new(i + 1);
                batch.on(true);
                batch.brightness(1);
                batch.saturation(1);
                batch.transition_time(1);
                batch.build()
            };
            client.set_light_state(light_i, &params);
        }

        loop {
            let tt: u64 = rng.gen_range(75, 101);
            let mut set = std::collections::HashSet::new();
            for (i, _) in lights.iter().enumerate() {
                let mut batch = HueLightBatch::new(i + 1);
                let hue_mod = rng.gen_range(std::u16::MAX / 100 / 32, std::u16::MAX / 100 / 20);
                let mut get_hue = || -> u16 {
                    if rng.gen_bool(0.5) {
                        hue.wrapping_add(hue_mod)
                    } else {
                        hue.wrapping_sub(hue_mod)
                    }
                };
                let mut hue = get_hue();
                while !set.insert(hue) {
                    hue = get_hue();
                }
                hue = hue.wrapping_mul(100);
                batch.hue(hue);
                batch.saturation(rng.gen_range(175, 255));
                batch.brightness(rng.gen_range(150, 255));
                batch.transition_time(50);
                let (light_i, params) = batch.build();
                client.set_light_state(light_i, &params);
            }
            hue = hue.wrapping_add(rng.gen_range(std::u16::MAX / 100 / 8, std::u16::MAX / 100 / 4));
            while hue > std::u16::MAX / 100 {
                hue -= std::u16::MAX / 100;
            }
            std::thread::sleep(Duration::from_millis(tt * 100));
        }
    }
}
