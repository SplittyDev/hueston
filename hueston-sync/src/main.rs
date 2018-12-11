// Import crates
#[macro_use]
extern crate error_chain;
use clap::clap_app;
use hueston::HueLightBatch;
use human_panic::setup_panic;

// Import std stuff
use std::fs::File;

// Import modules
mod discovery;
mod effects;
mod errors;
mod state;
use self::discovery::{discover_bridges, DiscoveryMode};
use self::effects::{ColorsweepEffect, FireplaceSimulation};
use self::errors::*;
use self::state::StartupState;

/// Try to load the startup state from disk.
fn try_read_startup_state() -> Option<StartupState> {
    let file = File::open(".hueston-sync.conf").ok()?;
    serde_json::from_reader(file).ok()
}

/// Load the startup state from disk or return a default state.
fn read_startup_state() -> StartupState {
    try_read_startup_state().unwrap_or_else(StartupState::default)
}

/// Main entry point.
fn main() -> self::errors::Result<()> {
    // Setup human-friendly error handing
    setup_panic!();

    // Parse command-line arguments
    let matches = clap_app!(hueston_sync =>
        (version: "0.0.1")
        (author: "Marco Quinten <splittydev@gmail.com>")
        (about: "Philips Hue remote control")

        // Light control
        (@subcommand light =>
            (@arg light: +takes_value -l --light "Choose the light")
            (@arg hue: +takes_value -h --hue "Set hue")
            (@arg sat: +takes_value -s --sat "Set saturation")
            (@arg bri: +takes_value -b --bri "Set brightness")
            (@arg tt: +takes_value --tt "Set transition time")
        )

        // Simulations
        (@subcommand simulate =>

            // Fireplace simulation
            (@subcommand fireplace =>
                (@arg bri_bas: +takes_value --bb "Brightness base")
                (@arg bri_var: +takes_value --bv "Brightness variance")
                (@arg sat_bas: +takes_value --sb "Saturation base")
                (@arg sat_var: +takes_value --sv "Saturation variance")
                (@arg tra_bas: +takes_value --tb "Transition time base")
                (@arg tra_var: +takes_value --tv "Transition time variance")
                (@arg spa_bri_bas: +takes_value --sbb "Spark brightness base")
                (@arg spa_bri_var: +takes_value --sbv "Spark brightness variance")
                (@arg spa_sat_bas: +takes_value --ssb "Spark saturation base")
                (@arg spa_sat_var: +takes_value --ssv "Spark saturation variance")
                (@arg spa_pro: +takes_value --sp "Spark probability")
            )

            // Color sweep effect
            (@subcommand colorsweep =>
            )
        )
    )
    .get_matches();

    // Load startup state from disk
    let mut state = read_startup_state();

    // Discover bridges if necessary
    discover_bridges(&mut state, DiscoveryMode::Initial);

    // Create bridge clients from saved state
    let clients = state
        .connect_bridge_clients()
        .chain_err(|| "Unable to connect bridge clients.")?;

    // Iterate over bridges
    for (i, client) in clients.iter().enumerate() {
        println!("[Bridge {}] {}", i + 1, client.get_name());

        // Test whether the simulate command was specified
        if let Some(matches) = matches.subcommand_matches("simulate") {
            // Fetch lights
            let lights = client.fetch_lights().unwrap();

            // Test whether the fireplace simulation was requested
            if let Some(matches) = matches.subcommand_matches("fireplace") {
                /// A macro for quick argument extraction
                macro_rules! extract {
                    ($name:ident || $default:expr) => {
                        matches
                            .value_of(stringify!($name))
                            .map(|v| v.parse().unwrap_or($default))
                            .unwrap_or($default)
                    };
                }

                // Construct the fireplace simulation
                let sim = FireplaceSimulation {
                    brightness_base: extract!(bri_bas || 70),
                    brightness_variance: extract!(bri_var || 10),
                    brightness_spark_base: extract!(bri_spa_bas || 110),
                    brightness_spark_variance: extract!(bri_spa_var || 10),
                    saturation_base: extract!(sat_bas || 247),
                    saturation_variance: extract!(sat_var || 7),
                    saturation_spark_base: extract!(sat_spa_bas || 227),
                    saturation_spark_variance: extract!(sat_spa_var || 27),
                    transition_time_base: extract!(tra_bas || 3),
                    transition_time_variance: extract!(tra_var || 2),
                    spark_probability: extract!(spa_pro || 0.01),
                };

                // Run the simulation
                sim.run(&client, lights);
            }
            // Test whether the color-sweep effect was requested
            else if let Some(_matches) = matches.subcommand_matches("colorsweep") {
                // Run the effect
                let effect = ColorsweepEffect {};
                effect.run(&client, lights);
            }
        }
        // Test whether the light command was specified
        else if let Some(matches) = matches.subcommand_matches("light") {
            // Fetch lights
            let lights = client.fetch_lights().unwrap();

            // Iterate over all lights
            for (j, _) in lights.iter().enumerate() {
                let mut batch = HueLightBatch::new(j + 1);
                macro_rules! batch_op {
                    ($fun:ident, $op:ident) => {
                        if let Some(val) = matches.value_of(stringify!($op)) {
                            batch.$fun(val.parse().unwrap());
                        }
                    };
                }
                batch_op!(brightness, bri);
                batch_op!(saturation, sat);
                batch_op!(hue, hue);
                batch_op!(transition_time, tt);
                let (light_i, params) = batch.build();
                client.set_light_state(light_i, &params);
            }
        }
    }
    Ok(())
}
