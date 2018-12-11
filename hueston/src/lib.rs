pub mod hue_bridge;
pub use self::hue_bridge::HueBridge;

pub mod hue_bridge_client;
pub use self::hue_bridge_client::HueBridgeClient;

pub mod hue_light;
pub use self::hue_light::{HueLight, HueLightBatch, HueLightState};

mod hue_resp;
pub use self::hue_resp::HueErrorCode;

/// Hueston.
pub struct Hueston;

impl Hueston {
    /// Discover all Hue Bridges.
    pub fn discover_bridges() -> Option<Vec<HueBridgeClient>> {
        // Discover all Hue Bridge devices
        let bridges = match HueBridge::discover_all() {
            Some(vec) => vec,
            None => return None,
        };

        // Map the Hue Bridge devices to Hue Bridge clients
        let vec: Vec<HueBridgeClient> = bridges.into_iter().map(HueBridgeClient::new).collect();

        // Return the Hue Bridge clients
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }
}
