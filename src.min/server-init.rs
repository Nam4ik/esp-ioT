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
    http::server::{Configuration as HttpConfig, EspHttpServer},
    nvs::{EspNvs, Nvs},
    wifi::{WifiMode, EspWifi},
};
use esp_wifi::wifi::WiFiModem;
use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WifiConfig {
    pub ssid: String<32>,
    pub password: String<64>,
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
    pub condition: String<20>,
    pub threshold: f32,
    pub action: String<20>,
}

static HTML_PAGE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>ESP32 Control</title>
    <script>
        async function saveConfig() {
            const ssid = document.getElementById('ssid').value;
            const password = document.getElementById('password').value;
            
            await fetch('/config', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: `ssid=${encodeURIComponent(ssid)}&password=${encodeURIComponent(password)}`
            });
            alert('Config saved! Rebooting...');
        }

        async function addRule() {
            const condition = document.getElementById('condition').value;
            const threshold = document.getElementById('threshold').value;
            const action = document.getElementById('action').value;
            
            await fetch('/rules', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    condition,
                    threshold: parseFloat(threshold),
                    action
                })
            });
            alert('Rule added!');
        }
    </script>
</head>
<body>
    <h1>WiFi Configuration</h1>
    <div>
        <input type="text" id="ssid" placeholder="SSID">
        <input type="password" id="password" placeholder="Password">
        <button onclick="saveConfig()">Save WiFi</button>
    </div>

    <h2>Add Rule</h2>
    <div>
        <select id="condition">
            <option value="light_high">Light High</option>
            <option value="light_low">Light Low</option>
        </select>
        <input type="number" id="threshold" step="0.1" placeholder="Threshold">
        <select id="action">
            <option value="relay_off">Turn Off Relay</option>
            <option value="relay_on">Turn On Relay</option>
        </select>
        <button onclick="addRule()">Add Rule</button>
    </div>
</body>
</html>
"#;

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

pub fn save_config(nvs: &mut EspNvs<Nvs>, config: &WifiConfig) -> Result<()> {
    let bytes = postcard::to_vec::<_, 128>(config)?;
    nvs.set_str("wifi_config", core::str::from_utf8(&bytes)?)?;
    Ok(())
}

pub fn start_webserver() -> Result<EspHttpServer, anyhow::Error> {
    let mut server = EspHttpServer::new(&HttpConfig {
        max_open_sockets: 3,
        ..Default::default()
    })?;

    server.fn_handler("/", Method::Get, |request| {
        request.into_ok_response()?.write_all(HTML_PAGE.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/config", Method::Post, |mut request| {
        let mut buf = [0u8; 128];
        let len = request.read(&mut buf)?;
        let body = core::str::from_utf8(&buf[..len])?;
        
        let mut ssid = String::new();
        let mut password = String::new();
        
        for (key, value) in form_urlencoded::parse(body.as_bytes()) {
            match key.as_ref() {
                "ssid" => ssid.push_str(&value).unwrap(),
                "password" => password.push_str(&value).unwrap(),
                _ => (),
            }
        }
        
        let config = WifiConfig { ssid, password };
        let mut nvs = init_nvs();
        save_config(&mut nvs, &config)?;
        
        request.into_response(Status::SeeOther)?.write_all(b"Restarting...")?;
        unsafe { esp_idf_sys::esp_restart() };
        Ok(())
    })?;

    Ok(server)
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
