//192.168.4.1
/*
#![no_main]
#![no_std]
#![feature(panic_info_message)]
*/

#![no_std]

use core::panic::PanicInfo;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    http::server::{Configuration as HttpConfig, EspHttpServer, Method, Status},
    nvs::{EspNvs, Nvs},
    wifi::{WifiMode, EspWifi},
};
use esp_wifi::wifi::WiFiModem;
use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

mod shared;
use shared::{Rule, SensorData, SharedState, WifiConfig};

use core::fmt;
use esp_idf_sys::EspError;
use spin::{Mutex, Once};
use core::sync::atomic::{AtomicBool, Ordering};

static HTML_PAGE: &str = include_str!("../mainpage.html");
static SHARED_STATE: Once<Mutex<SharedState>> = Once::new();
static RELAY_STATE: AtomicBool = AtomicBool::new(false);

pub struct Wifi {
    inner: EspWifi,
}

impl Wifi {
    pub fn init(modem: WiFiModem, config: Option<WifiConfig>) -> Result<Self, esp_wifi::Error> {
        let mut wifi = EspWifi::new(modem, None, None)?;
        
        if let Some(config) = config {
            wifi.set_mode(WifiMode::Sta)?;
            let sta_config = esp_wifi::wifi::StaConfig {
                ssid: config.ssid.into(),
                password: config.password.into(),
                ..Default::default()
            };
            wifi.set_config(&esp_wifi::wifi::Config::Sta(&sta_config))?;
            wifi.start()?;
            wifi.connect()?;
            
            for _ in 0..5 {
                if wifi.is_connected()? {
                    return Ok(Self { inner: wifi });
                }
                FreeRtos::delay_ms(1000);
            }
        }
        
        wifi.set_mode(WifiMode::Ap)?;
        let ap_config = esp_wifi::wifi::ApConfig {
            ssid: "ESP32-Config".into(),
            ..Default::default()
        };
        wifi.set_config(&esp_wifi::wifi::Config::Ap(&ap_config))?;
        wifi.start()?;
        
        Ok(Self { inner: wifi })
    }

    pub fn is_connected(&self) -> bool {
        self.inner.is_connected().unwrap_or(false)
    }

    pub fn is_ap(&self) -> bool {
        matches!(self.inner.get_mode(), Ok(WifiMode::Ap))
    }
}

pub fn init_nvs() -> EspNvs<Nvs> {
    EspNvs::new_default().unwrap()
}

pub fn load_config(nvs: &mut EspNvs<Nvs>) -> Option<WifiConfig> {
    if let Ok(Some(config_str)) = nvs.get_str("wifi_config") {
        postcard::from_bytes(config_str.as_bytes()).ok()
    } else {
        None
    }
}

pub fn save_config(nvs: &mut EspNvs<Nvs>, config: &WifiConfig) -> Result<(), EspError> {
    let bytes = postcard::to_vec::<_, 128>(config).map_err(|_| EspError::from(0))?;
    nvs.set_str("wifi_config", core::str::from_utf8(&bytes).map_err(|_| EspError::from(0))?)?;
    Ok(())
}

pub fn start_webserver(shared_state: &'static Mutex<SharedState>) -> Result<EspHttpServer, anyhow::Error> {
    SHARED_STATE.call_once(|| shared_state.clone());
    
    let mut server = EspHttpServer::new(&HttpConfig {
        max_open_sockets: 5,
        stack_size: 8192,
        ..Default::default()
    })?;

    server.fn_handler("/", Method::Get, |request| {
        request.into_ok_response()?.write_all(HTML_PAGE.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/api/data", Method::Get, |request| {
        let shared = SHARED_STATE.get().unwrap().lock();
        let json = serde_json_core::to_string::<SensorData, 256>(&shared.sensor_data)
            .map_err(|_| anyhow::anyhow!("Serialization failed"))?;
        
        request
            .into_ok_response()?
            .content_type("application/json")?
            .write_all(json.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/api/config", Method::Post, |mut request| {
        let mut buf = [0u8; 256];
        let len = request.read(&mut buf)?;
        let body = core::str::from_utf8(&buf[..len])?;
        
        let mut ssid = String::<32>::new();
        let mut password = String::<64>::new();
        
        for (key, value) in form_urlencoded::parse(body.as_bytes()) {
            match key.as_ref() {
                "ssid" => ssid.push_str(&value).map_err(|_| anyhow::anyhow!("SSID too long"))?,
                "password" => password.push_str(&value).map_err(|_| anyhow::anyhow!("Password too long"))?,
                _ => (),
            }
        }
        
        if ssid.is_empty() || password.is_empty() {
            return request.into_response(Status::BadRequest)?.write_all(b"Invalid input");
        }
        
        let config = WifiConfig { ssid, password };
        let mut nvs = init_nvs();
        save_config(&mut nvs, &config)?;
        
        request.into_response(Status::SeeOther)?.write_all(b"Restarting...")?;
        unsafe { esp_idf_sys::esp_restart() };
        Ok(())
    })?;

    server.fn_handler("/api/rules", Method::Post, |mut request| {
        let mut buf = [0u8; 128];
        let len = request.read(&mut buf)?;
        let body = core::str::from_utf8(&buf[..len])?;
        
        let rule: Rule = serde_json_core::from_str(body)
            .map_err(|_| anyhow::anyhow!("Deserialization failed"))?
            .0;
        
        let mut shared = SHARED_STATE.get().unwrap().lock();
        shared.rules.push(rule).map_err(|_| anyhow::anyhow!("Rules full"))?;
        
        request.into_ok_response()?.write_all(b"Rule added")?;
        Ok(())
    })?;

    server.fn_handler("/api/relay", Method::Post, |mut request| {
        let mut buf = [0u8; 16];
        let len = request.read(&mut buf)?;
        let body = core::str::from_utf8(&buf[..len])?;
        
        let state = match body {
            "on" => true,
            "off" => false,
            _ => return request.into_response(Status::BadRequest)?.write_all(b"Invalid state"),
        };
        
        RELAY_STATE.store(state, Ordering::SeqCst);
        request.into_ok_response()?.write_all(b"Relay updated")?;
        Ok(())
    })?;

    Ok(server)
}

pub fn get_relay_state() -> bool {
    RELAY_STATE.load(Ordering::SeqCst)
}

pub fn apply_rules(sensor_data: &SensorData) -> bool {
    let shared = SHARED_STATE.get().unwrap().lock();
    
    for rule in &shared.rules {
        let condition_met = match rule.condition.as_str() {
            "light_high" => sensor_data.light > rule.threshold,
            "light_low" => sensor_data.light < rule.threshold,
            "temp_high" => sensor_data.temperature > rule.threshold,
            "temp_low" => sensor_data.temperature < rule.threshold,
            _ => false,
        };
        
        if condition_met {
            return match rule.action.as_str() {
                "relay_on" => true,
                "relay_off" => false,
                _ => get_relay_state(),
            };
        }
    }
    
    get_relay_state()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log::error!(
            "Panic at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap_or(&format_args!(""))
        );
    }
    loop {
        unsafe { esp_idf_sys::esp_restart() };
    }
}
