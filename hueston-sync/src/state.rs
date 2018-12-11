use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use super::errors::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConnectionInfo {

    /// The bridge host.
    /// This field is badly named, it's actually
    /// a URL and not an IP address.
    pub ip: String,

    /// The bridge username for authentication.
    pub username: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StartupState {

    /// The Hue bridge devices.
    pub bridges: Option<Vec<BridgeConnectionInfo>>,
}

impl StartupState {

    /// Save the startup state to disk.
    pub fn save_to_disk(&self) -> Result<()> {
        let file = File::create(".hueston-sync.conf")
            .chain_err(|| "Unable to open '.hueston-sync.conf'.")?;
        serde_json::to_writer(file, &self)
            .chain_err(|| "Unable to write state to file.")?;
        Ok(())
    }

    /// Connect bridge clients.
    pub fn connect_bridge_clients(&self) -> Option<Vec<hueston::HueBridgeClient>> {

        // Make sure we found some bridges
        let bridges = self.bridges.as_ref()?;

        // Fetch bridge clients
        Some(bridges
            .iter()
            .map(|info| {
                let username = info.username.as_ref().map(ToString::to_string);
                let bridge = hueston::HueBridge::from_url(&info.ip);
                bridge.map(|bridge| bridge.with_username(username))
            })
            .filter_map(|bridge| bridge)
            .map(hueston::HueBridgeClient::new)
            .collect()
        )
    }

    /// Add a bridge to the state.
    pub fn add_bridge(&mut self, info: BridgeConnectionInfo) {
        if let Some(bridges) = &mut self.bridges {
            bridges.push(info);
        } else {
            self.bridges = Some(vec![info]);
        }
    }

    /// Test whether the state contains any bridges.
    pub fn has_bridges(&self) -> bool {
        self.bridges.is_some()
    }

}

impl Into<BridgeConnectionInfo> for &hueston::HueBridgeClient {
    fn into(self) -> BridgeConnectionInfo {
        BridgeConnectionInfo {
            ip: self.get_url().to_string(),
            username: self.get_username().cloned(),
        }
    }
} 