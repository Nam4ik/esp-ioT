// main.rs
#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod wifi;
mod ioT;

use anyhow::Result;
use esp_idf_hal::{gpio::AnyIOPin, peripherals::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::{info, error};

use crate::iot::{IoTConfig, IoTContext};

#[entry]
fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;


    let mut wifi = wifi::init_wifi(
        peripherals.modem,
        sysloop,
        nvs,
    )?;

/*
    let iot_config = IoTConfig {
        server_url: "http://your-server.com/api/data".into(),
        sampling_interval: 5000, //5s
    };
*/

    let adc_pin = peripherals.pins.gpio32.into();
    let mut iot = IoTContext::new(adc_pin, iot_config)?;


    loop {
        if wifi.is_connected() {
            ioT.run()?;
        } else {
            wifi.reconnect()?;
            esp_idf_hal::delay::FreeRtos::delay_ms(1000);
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("Panic: {:?}", info);
    loop {
        unsafe { esp_idf_sys::esp_restart() };
    }
}