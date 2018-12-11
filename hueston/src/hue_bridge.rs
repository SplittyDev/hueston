use std::io::Read;

use ssdp::header::{HeaderRef, HeaderMut};
use ssdp::message::{SearchRequest, Multicast};

use serde_derive::Deserialize;
use serde_xml_rs::deserialize;

/// Hue Bridge Device.
#[derive(Deserialize, Debug)]
pub struct HueBridgeDevice {
    #[serde(rename = "friendlyName")]
    friendly_name: String,
    #[serde(rename = "modelName")]
    model_name: String,
    #[serde(rename = "serialNumber")]
    serial_number: String,
    #[serde(rename = "UDN")]
    udn: String,
}

/// Hue Bridge.
#[derive(Deserialize, Debug)]
pub struct HueBridge {
    #[serde(rename = "URLBase")]
    url_base: String,
    device: HueBridgeDevice,
    username: Option<String>,
}

impl HueBridge {

    /// Get the friendly name.
    pub fn get_name(&self) -> &String {
        &self.device.friendly_name
    }

    /// Get the base URL.
    pub fn get_url(&self) -> &String {
        &self.url_base
    }

    /// Get the model name.
    pub fn get_model(&self) -> &String {
        &self.device.model_name
    }

    /// Get the username.
    pub fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    /// Set the username.
    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    /// Get an API endpoint relative to the base
    pub fn get_endpoint(&self, ep: &str) -> String {
        format!(
            "{base}/api{path}",
            base = self.url_base.trim_end_matches('/'),
            path = ep,
        )
    }

    /// Get an authenticated API endpoint relative to the base
    pub fn get_auth_endpoint(&self, username: &str, ep: &str) -> String {
        format!(
            "{base}/api/{username}/{path}",
            username = username,
            base = self.url_base.trim_end_matches('/'),
            path = ep.trim_start_matches('/'),
        )
    }

    /// Discover Hue Bridge devices using various methods.
    /// 
    /// At the moment, only UPnP discovery is supported.
    pub fn discover_all() -> Option<Vec<HueBridge>> {
        Self::discover_upnp()
    }

    /// Discover Hue Bridge devices via UPnP.
    fn discover_upnp() -> Option<Vec<HueBridge>> {

        // Create a vector of hue bridges
        let mut vec = Vec::<HueBridge>::new();

        // Create a set of hue bridge locations
        let mut found = std::collections::HashSet::<String>::new();

        // Create the UPnP search request
        let mut req = SearchRequest::new();
        req.set(ssdp::header::Man);
        req.set(ssdp::header::MX(5));

        let mut process_response = |msg: ssdp::message::SearchResponse| -> Option<HueBridge> {

            // Extract the server header
            let server = msg.get::<ssdp::header::Server>()?;

            // Test whether the server contains "IpBridge"
            if !server.as_str().contains("IpBridge") { return None }

            // Extract the location header
            let location = msg
                .get::<ssdp::header::Location>()?
                .as_str()
                .to_string();
            
            // Continue if the location was already found
            if found.contains(&location) { return None }

            // Fetch the bridge configuration
            let mut resp = reqwest::get(&location).ok()?;

            // Insert the location into the set
            found.insert(location);

            // Read the bridge configuration into a string
            let mut content = String::new();
            resp.read_to_string(&mut content).ok()?;

            // Deserialize the bridge configuration
            deserialize(content.as_bytes()).ok()?
        };

        // Iterate over all search responses
        for (msg, _) in req.multicast().unwrap() {
            if let Some(bridge) = process_response(msg) {
                vec.push(bridge);
            }
        }

        // Return the discovered bridges
        if vec.is_empty() { None } else { Some(vec) }
    }

    pub fn from_url(url: &str) -> Option<HueBridge> {
        let url = &format!("{}description.xml", url);
        
        // Fetch the bridge configuration
        let mut resp = reqwest::get(url).ok()?;

        // Read the bridge configuration into a string
        let mut content = String::new();
        resp.read_to_string(&mut content).ok()?;

        // Deserialize the bridge configuration
        deserialize(content.as_bytes()).ok()?
    }

    pub fn with_username(mut self, username: Option<String>) -> Self {
        self.username = username;
        self
    }
}