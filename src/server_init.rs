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

static HTML_PAGE: &str = include_str("./mainpage.html")?;

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


    server.fn_handler("/upload", Method::Post, |mut request| {
        let mut file = fs.open_file_write("/uploaded_file.json")?;
        let mut buffer = [0u8; 1024];
        while let Ok(len) = request.read(&mut buffer) {
            file.write(&buffer[..len])?;
    }
        request.into_ok_response()?.write_all(b"File uploaded")?;
        Ok(())
})?;
        
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
