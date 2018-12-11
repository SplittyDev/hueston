use hueston::{HueBridgeClient, HueLight, HueLightBatch};
use rand::prelude::*;
use std::time::Duration;

pub struct FireplaceSimulation {
    pub brightness_base: u8,
    pub brightness_variance: u8,
    pub brightness_spark_base: u8,
    pub brightness_spark_variance: u8,
    pub saturation_base: u8,
    pub saturation_variance: u8,
    pub saturation_spark_base: u8,
    pub saturation_spark_variance: u8,
    pub transition_time_base: u8,
    pub transition_time_variance: u8,
    pub spark_probability: f64,
}

impl Default for FireplaceSimulation {
    fn default() -> Self {
        Self {
            brightness_base: 70,
            brightness_variance: 10,
            brightness_spark_base: 110,
            brightness_spark_variance: 10,
            saturation_base: 247,
            saturation_variance: 7,
            saturation_spark_base: 227,
            saturation_spark_variance: 27,
            transition_time_base: 3,
            transition_time_variance: 2,
            spark_probability: 0.01,
        }
    }
}

impl FireplaceSimulation {
    pub fn run(&self, client: &HueBridgeClient, lights: Vec<HueLight>) {
        let (send, recv) = std::sync::mpsc::channel();

        // Calculate boundaries
        let brightness_min = self
            .brightness_base
            .saturating_sub(self.brightness_variance)
            .max(1);
        let brightness_max = self
            .brightness_base
            .saturating_add(self.brightness_variance.saturating_add(1))
            .min(255);
        let brightness_spark_min = self
            .brightness_spark_base
            .saturating_sub(self.brightness_spark_variance)
            .max(1);
        let brightness_spark_max = self
            .brightness_spark_base
            .saturating_add(self.brightness_spark_variance.saturating_add(1))
            .min(255);
        let saturation_min = self
            .saturation_base
            .saturating_sub(self.saturation_variance)
            .max(1);
        let saturation_max = self
            .saturation_base
            .saturating_add(self.saturation_variance.saturating_add(1))
            .min(255);
        let saturation_spark_min = self
            .saturation_spark_base
            .saturating_sub(self.saturation_spark_variance)
            .max(1);
        let saturation_spark_max = self
            .saturation_spark_base
            .saturating_add(self.saturation_spark_variance.saturating_add(1))
            .min(255);
        let transition_min = self
            .transition_time_base
            .saturating_sub(self.transition_time_variance)
            .max(0);
        let transition_max = self
            .transition_time_base
            .saturating_add(self.transition_time_variance.saturating_add(1))
            .min(255);
        let spark_probability = self.spark_probability.min(1.0).max(0.0);

        // Iterate over all lights
        for (i, light) in lights.iter().enumerate() {
            let send = send.clone();

            // Turn the light on
            if !light.is_on() {
                let (light_i, params) = {
                    let mut batch = HueLightBatch::new(i + 1);
                    batch.on(true);
                    batch.build()
                };
                client.set_light_state(light_i, &params);
            }

            // Spawn a handling thread for the current light
            std::thread::spawn(move || {
                let mut rng = rand::thread_rng();
                loop {
                    let spark = rng.gen_bool(spark_probability);
                    let tt: u16 = if spark {
                        0
                    } else {
                        u16::from(rng.gen_range(transition_min, transition_max))
                    };

                    // Prepare the command batch
                    let batch = {
                        let mut batch = HueLightBatch::new(i + 1);
                        if spark {
                            batch.hue(rng.gen_range(0, 80) * 100);
                            batch.saturation(
                                rng.gen_range(saturation_spark_min, saturation_spark_max),
                            );
                            batch.brightness(
                                rng.gen_range(brightness_spark_min, brightness_spark_max),
                            );
                        } else {
                            batch.hue(rng.gen_range(20, 70) * 100);
                            batch.saturation(rng.gen_range(saturation_min, saturation_max));
                            batch.brightness(rng.gen_range(brightness_min, brightness_max));
                        }
                        batch.transition_time(tt);
                        batch
                    };

                    // Serialize the parameters
                    let (light_i, params) = batch.build();
                    let json = serde_json::to_string(&params).unwrap();

                    // Send the parameters through the channel
                    send.send((light_i, json)).unwrap();

                    // Sleep for the transition time
                    let dur = Duration::from_millis(u64::from(tt) * 100);
                    std::thread::sleep(dur);
                }
            });
        }

        // Receive parameters
        while let Ok((light_i, params)) = recv.recv() {
            client.set_light_state_str(light_i, &params);
        }
    }
}
