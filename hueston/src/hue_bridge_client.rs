use super::HueBridge;
use super::HueLight;
use super::hue_resp::{
    HueErrorCode,
    HueBridgeRegistration,
};

use maplit::hashmap;
use std::collections::BTreeMap;

/// Hue Bridge.
#[derive(Debug)]
pub struct HueBridgeClient {
    bridge: HueBridge,
    client: reqwest::Client,
}

impl HueBridgeClient {

    /// Construct a new `HueBridgeClient` from a `HueBridge`.
    pub fn new(bridge: HueBridge) -> Self {
        Self {
            bridge,
            client: reqwest::Client::new()
        }
    }

    /// Get the API username
    pub fn get_username(&self) -> Option<&String> {
        self.bridge.get_username()
    }

    pub fn set_light_saturation(&self, i: usize, sat: u8) {
        let username = &self.get_username().unwrap();
        let ep = self.get_auth_endpoint(username, &format!("/lights/{}/state", i));
        let params = hashmap!{
            "sat" => sat.max(1).min(254)
        };
        let req = self.client.put(&ep).json(&params);
        let mut res = req.send().ok().unwrap();
        let data = res.text().ok().unwrap();
        println!("{:?}", data);
    }

    pub fn set_light_state(&self, i: usize, state: &std::collections::HashMap<&'static str, serde_json::Value>) {
        let username = &self.get_username().unwrap();
        let ep = self.get_auth_endpoint(username, &format!("/lights/{}/state", i));
        let req = self.client.put(&ep).json(&state);
        let mut res = req.send().ok().unwrap();
        let data = res.text().ok().unwrap();
    }

    pub fn set_light_state_str(&self, i: usize, state: &str) {
        let username = &self.get_username().unwrap();
        let ep = self.get_auth_endpoint(username, &format!("/lights/{}/state", i));
        let json: serde_json::Value = serde_json::from_str(state).unwrap();
        let req = self.client.put(&ep).json(&json);
        let mut res = req.send().ok().unwrap();
        let data = res.text().ok().unwrap();
    }

    pub fn fetch_lights(&self) -> Option<Vec<HueLight>> {
        let username = &self.get_username()?;
        let ep = self.get_auth_endpoint(username, "/lights");
        let req = self.client.get(&ep);
        let mut res = req.send().ok()?;
        let hm: BTreeMap<usize, HueLight> = res.json().ok()?;
        Some(hm.into_iter().map(|(_, v)| v).collect())
    }

    pub fn register(&mut self, app: &str) -> Result<(), HueErrorCode> {

        // Get the API endpoint
        let ep = self.get_endpoint("");

        // Build parameters
        let params = hashmap! {
            "devicetype" => format!("Hueston#{}", app),
        };

        // Prepare POST request
        let req = self.client.post(&ep).json(&params);

        // Send request
        let mut res = match req.send() {
            Ok(res) => res,
            Err(_) => return Err(HueErrorCode::Unknown),
        };

        // Deserialize response
        let data: Vec<HueBridgeRegistration> = match res.json() {
            Ok(data) => data,
            Err(err) => {
                println!("{}", err);
                return Err(HueErrorCode::Unknown)
            }
        };

        // Iterate over response data
        let mut last_error = None;
        for status in data {

            // Process the response
            match status.into_result() {
                Ok(resp) => {
                    self.bridge.set_username(resp.username);
                    return Ok(())
                },
                Err(err) => last_error = Some(err.error_code()),
            }
        }

        // Return the last error
        Err(last_error.unwrap_or(HueErrorCode::Unknown))
    }
}

impl std::ops::Deref for HueBridgeClient {
    type Target = HueBridge;

    fn deref(&self) -> &HueBridge {
        &self.bridge
    }
}