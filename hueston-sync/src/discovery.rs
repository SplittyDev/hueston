use hueston::{Hueston, HueErrorCode};
use crate::state::StartupState;

pub enum DiscoveryMode {

    /// Discover all bridges.
    /// Return if bridges were already found.
    Initial,

    /// Discover new bridges.
    #[allow(dead_code)]
    Additional,
}

pub fn discover_bridges(state: &mut StartupState, mode: DiscoveryMode) {

    // Return if we already found bridges
    if let DiscoveryMode::Initial = mode {
        if state.has_bridges() {
            return
        }
    }

    // WIP
    if let DiscoveryMode::Additional = mode {
        unimplemented!("The 'Additional' discovery mode is yet to be implemented.");
    }

    // Discover bridges
    println!("Discovering Hue bridges...");
    let bridges = match Hueston::discover_bridges() {
        Some(vec) => vec,
        None => {
            println!("Unable to find Hue bridge.");
            return
        }
    };

    // Iterate over all bridges
    for mut bridge in bridges {
        println!("Found: {}", bridge.get_name());
        let mut waiting_for_confirmation = false;

        // Loop until the registration is done
        loop {

            // Match on the registration response
            match bridge.register("hueston-sync") {

                // All good
                Ok(_) => {

                    // Add the bridge to the state
                    state.add_bridge((&bridge).into());

                    // Reserialize the state
                    state.save_to_disk().unwrap();
                    break
                },

                // An error has occurred
                Err(code) => {

                    // Test whether the link button needs to be pressed
                    if let HueErrorCode::LinkButtonNotPressed = code {

                        // Prompt the user to press the link button
                        if !waiting_for_confirmation {
                            println!("Please press the link button on the bridge.");
                            waiting_for_confirmation = true;
                        }

                        // Wait 500ms and retry
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                    
                    // Some other error occurred
                    else {

                        // Exit, we cannot survive at this point
                        panic!("Unknown error. Exiting.");
                    }
                }
            }
        }
    }
}