// main.rs
mod wifi;
mod sensor;

use esp_idf_hal::prelude::*;
use esp_idf_svc::log::EspLogger;
use sensor::TemperatureSensor;
use wifi::*;
use sensor::*;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    let mut nvs = wifi::init_nvs();
    let config = wifi::load_config(&mut nvs);
    
    let peripherals = Peripherals::take()?;
    let wifi = wifi::init_wifi(peripherals.modem, config)?;

    // Инициализация датчика температуры
    let temp_pin = peripherals.pins.gpio36.into();
    let mut sensor = TemperatureSensor::new(temp_pin)?;

    if wifi.is_ap() {
        let _server = wifi::start_webserver()?;
        esp_println::println!("AP Mode: Connect to ESP32-Config");
    } else {
        esp_println::println!("Connected to WiFi!");
    }

    loop {
        match sensor.read_celsius() {
            Ok(temp) => {
                esp_println::println!("Temperature: {:.2}°C", temp);
                
                // Здесь можно добавить отправку данных на сервер
                // send_to_server(temp)?;
            },
            Err(e) => esp_println::eprintln!("Sensor error: {:?}", e),
        }
        
        FreeRtos::delay_ms(5000);
    }
}
