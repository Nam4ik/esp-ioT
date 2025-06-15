use serde::{Serialize, Deserialize};
use heapless::{String, Vec};

#[derive(Serialize, Deserialize, Clone)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rule {
    pub condition: String,
    pub threshold: f32,
    pub action: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SensorData {
    pub voltage: f32,
    pub temperature: f32,
    pub light: f32,
    pub distance: f32,
}

#[derive(Clone)]
pub struct SharedState {
    pub sensor_data: SensorData,
    pub rules: Vec<Rule, 8>,
//  pub relay_state: bool,
}
