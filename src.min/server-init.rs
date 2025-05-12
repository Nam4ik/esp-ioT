//192.168.4.1

#![no_main]
#![no_std]
#![feature(panic_info_message)]
/*
#![feature(lang_items)]

#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
*/

use core::{panic::PanicInfo, marker::Sized, result::Result};
use esp_idf_hal::{
    gpio::{AnyIOPin, Pin, InterruptType, Pull},
    adc::{Adc, Attenuation, AdcChannelDriver, ADC1},
    peripherals::Peripherals,
    delay::FreeRtos,
};
use esp_idf_sys::{self as _, gpio_num_t, ESP_LOG_INFO};
use esp_wifi::{EspWifi, WiFiModem, current_millis};
use serde::{Deserialize, Serialize};
use core::ffi::VaList;
use esp_idf_svc::nvs::{EspNvs, Nvs};
use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpConfig};
use embedded_svc::http::{Method, Status};
use embedded_svc::io::Write;
use heapless::{String, Vec};
use esp_idf_svc::wifi::{WifiMode, self};
//use critical_section::implementation;

#[derive(Deserialize, Serialize)]
struct WifiConfig {
    ssid: heapless::String<32>,
    password: heapless::String<64>,
}

static HTML_PAGE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>WiFi Setup</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <h1>WiFi Configuration</h1>
    <form action="/config" method="post">
        <label>SSID:</label>
        <input type="text" name="ssid" required><br>
        <label>Password:</label>
        <input type="password" name="password"><br>
        <button type="submit">Save</button>
    </form>
</body>
</html>
"#;

#![allow(non_camel_case_types)]
type VaargType = core::ffi::VaList;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        esp_println::panic_log!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        );
    }
    loop {
        unsafe { esp_idf_sys::esp_restart() };
    }
}

/*
#[critical_section::implementation]
unsafe fn critical_section_impl(cs: critical_section::RawRestoreState) -> critical_section::RawRestoreState {
    let prev = unsafe { esp_idf_hal::cpu::disable_interrupts() };
    cs.set(prev as u32);
    prev as u32
}
*/ 
impl WebServer {

fn init_nvs() -> EspNvs<Nvs> {
    EspNvs::new_default().unwrap()
}

fn load_config(nvs: &mut EspNvs<Nvs>) -> Option<WifiConfig> {
    if let Ok(Some(config_str)) = nvs.get_str("wifi_config") {
        let bytes = config_str.as_bytes();
        let mut buffer = heapless::Vec::<u8, 128>::new();                                                   //
        buffer.extend_from_slice(bytes).ok()?;
        postcard::from_bytes(&buffer).ok()
    } else {
        None
    }
}

fn save_config(nvs: &mut EspNvs<Nvs>, config: &WifiConfig) -> anyhow::Result<()> {
    let mut buffer = heapless::Vec::<u8, 128>::new();
    postcard::to_vec(config, &mut buffer)?;
    let config_str = core::str::from_utf8(&buffer)?;
    nvs.set_str("wifi_config", config_str)?;
    Ok(())
}

fn start_webserver() -> anyhow::Result<EspHttpServer> {
    let mut server = EspHttpServer::new(&HttpConfig {
        max_open_sockets: 3,
        ..Default::default()
    })?;

    server.fn_handler("/", Method::Get, |request| {
        let mut response = request.into_ok_response()?;
        response.write_all(HTML_PAGE.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/config", Method::Post, |mut request| {
        let mut buf = [0u8; 256];
        let len = request.read(&mut buf)?;
        let body = core::str::from_utf8(&buf[..len])?;
        
        let config = form_urlencoded::parse(body.as_bytes())
            .filter_map(|(key, value)| {
                match key.as_ref() {
                    "ssid" => Some((key, value)),
                    "password" => Some((key, value)),
                    _ => None
                }
            })
            .fold(WifiConfig { ssid: String::new(), password: String::new() }, |mut acc, (key, value)| {
                match key.as_ref() {
                    "ssid" => acc.ssid = value.to_string(),
                    "password" => acc.password = value.to_string(),
                    _ => ()
                }
                acc
            });

        let mut nvs = init_nvs();
        save_config(&mut nvs, &config).unwrap();
        
        let mut response = request.into_response(Status::SeeOther)?;
        response.write_all(b"Configuration saved! Restarting...")?;
        unsafe { esp_idf_sys::esp_restart() };
        Ok(())
    })?;

    Ok(server)
}
}

impl Wifi {
pub fn init_wifi(wifi_modem: WiFiModem, config: Option<WifiConfig>) -> Result<EspWifi<'static>, esp_wifi::Error> {
    let mut wifi = EspWifi::new(wifi_modem, None, None)?;
    
    if let Some(config) = config {
        // STA mode with config
        wifi.set_mode(WiFiMode::STA)?;
        let mut sta_config = wifi::StaConfig {
            ssid: config.ssid.as_bytes().try_into().unwrap(),
            password: config.password.as_bytes().try_into().unwrap(),
            ..Default::default()
        };
        wifi.set_config(&wifi::Config::Sta(&sta_config))?;
        wifi.start()?;
        wifi.connect()?;
        
        let mut retries = 0;
        while !wifi.is_connected()? && retries < 5 {
            FreeRtos::delay_ms(1000);
            retries += 1;
        }
        
        if wifi.is_connected()? {
            return Ok(wifi);
        }
    }
    
    // Fallback to AP mode
    wifi.set_mode(WiFiMode::AP)?;
    let ap_config = wifi::ApConfig {
        ssid: "ESP32-Config".into(),
        channel: 1,
        ..Default::default()
    };
    wifi.set_config(&wifi::Config::Ap(&ap_config))?;
    wifi.start()?;
    Ok(wifi)
}

pub fn wifi_status -> bool {
    if(wifi.is_connected) {
        return true;
    }else {
        return false;
    }
}

/*
fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();
    
    let mut nvs = init_nvs();
    let config = load_config(&mut nvs);
    
    let peripherals = Peripherals::take().unwrap();
    let wifi = init_wifi(peripherals.modem, config)?;
    
    if wifi.is_ap() {
        let _server = start_webserver()?;
        esp_println::println!("AP mode started. Connect to SSID: ESP32-Config");
    } else {
        esp_println::println!("Connected to WiFi");
    }

    
    loop {
        FreeRtos::delay_ms(1000);
    }

    }
*/
}
