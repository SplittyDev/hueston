use serde_derive::Deserialize;
use std::collections::HashMap;

/// Hue Bridge Device.
#[derive(Deserialize, Debug)]
pub struct HueLight {

    /// State
    state: HueLightState,

    /// Light type
    #[serde(rename = "type")]
    r#type: String,

    /// Light name
    name: String,

    /// Hardware model
    #[serde(rename = "modelid")]
    model_id: String,

    /// Manufacturer name
    #[serde(rename = "manufacturername")]
    manufacturer_name: String,

    /// Product name
    #[serde(rename = "productname")]
    product_name: String,

    /// Unique ID
    #[serde(rename = "uniqueid")]
    unique_id: String,

    /// Software version
    #[serde(rename = "swversion")]
    sw_version: String,
}

impl HueLight {

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl std::ops::Deref for HueLight {
    type Target = HueLightState;

    fn deref(&self) -> &HueLightState {
        &self.state
    }
}

#[derive(Deserialize, Debug)]
pub struct HueLightState {

    /// State of the light
    on: bool,

    /// Brightness
    /// From 1 to 254
    bri: u8,

    /// Hue
    /// From 0 to 65535
    hue: u16,

    /// Saturation
    /// From 0 to 254
    sat: u8,

    /// Dynamic effect
    /// Either 'none' or 'colorloop'
    effect: String,

    /// Color coordinates in CIE color space
    xy: HueLightCoords,

    /// Color temperature
    ct: u16,

    /// Alert effect
    /// Either 'none', 'select' or 'lselect'
    alert: String,

    /// Color mode
    /// Either 'hs', 'xy' or 'ct'
    colormode: Option<String>,

    /// Whether the light is reachable
    reachable: bool,
}

impl HueLightState {
    pub fn is_on(&self) -> bool {
        self.on
    }
}

#[derive(Deserialize, Debug)]
pub struct HueLightCoords {
    x: f32,
    y: f32,
}

pub struct HueLightBatch {
    index: usize,
    map: HashMap<&'static str, serde_json::Value>
}

macro_rules! impl_batch_op {
    ($localname:ident : $type:ty) => (impl_batch_op!($localname => $localname : $type););
    ($localname:ident => $remotename:ident : $type:ty) => {
        pub fn $localname (&mut self, val: $type) -> &mut Self {
            let val = serde_json::to_value(val).unwrap();
            self.map.insert(stringify!($remotename), val);
            self
        }
    };
}

impl HueLightBatch {

    /// Construct a new `HueLightBatch`.
    pub fn new(light_index: usize) -> Self {
        Self {
            index: light_index,
            map: HashMap::new(),
        }
    }

    /// Get the light index and parameter HashMap.
    pub fn build(self) -> (usize, HashMap<&'static str, serde_json::Value>) {
        (self.index, self.map)
    }

    impl_batch_op!(on: bool);
    impl_batch_op!(temperature => ct: u16);
    impl_batch_op!(brightness => bri: u8);
    impl_batch_op!(saturation => sat: u8);
    impl_batch_op!(hue: u16);
    impl_batch_op!(transition_time => transitiontime: u16);
}