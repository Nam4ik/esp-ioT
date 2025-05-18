#![no_std]
#![no_main]

use core::fmt;
use anyhow::Result;
use esp_idf_hal::{
    adc::{Adc, AdcChannelDriver, AdcDriver, Attenuation, ADC1},
    gpio::{AnyIOPin, PinDriver},
    i2c::I2cDriver,
    peripherals::Peripherals,
    delay::FreeRtos,
    prelude::*
};
use esp_idf_sys as _;
use nanoserde::{Serialize, Deserialize};

mod server_init;
use server_init::wifi::{init_wifi, wifi_status, WifiConfig};

// Sensors
use bmp280::BMP280;
use bh1750::{BH1750, Resolution};

#[derive(Serialize, Deserialize)]
struct SensorData {
    voltage: f32,
    temperature: f32,
    light: f32,
    distance: f32,
}

struct Sensors {
    adc: AdcDriver<'static, ADC1>,
    adc_pin: AdcChannelDriver<'static, { Attenuation::Db11 }>,
    i2c: I2cDriver<'static>,
    trig_pin: PinDriver<'static, AnyIOPin, Output>,
    echo_pin: PinDriver<'static, AnyIOPin, Input>,
}

impl Sensors {
    fn new(peripherals: Peripherals) -> Result<Self> {
        let adc = AdcDriver::new(peripherals.adc1, &esp_idf_hal::config::Config::new().calibration(true))?;
        let adc_pin = AdcChannelDriver::new(peripherals.pins.gpio32.into_analog())?;
        
        let i2c = I2cDriver::new(
            peripherals.i2c0,
            peripherals.pins.gpio21.into(),
            peripherals.pins.gpio22.into(),
            &esp_idf_hal::i2c::I2cConfig::new().baudrate(100.kHz().into()),
        )?;

        let trig_pin = PinDriver::output(peripherals.pins.gpio5.into())?;
        let echo_pin = PinDriver::input(peripherals.pins.gpio6.into())?;

        Ok(Self {
            adc,
            adc_pin,
            i2c,
            trig_pin,
            echo_pin,
        })
    }

    fn read_voltage(&mut self) -> Result<f32> {
        let raw = self.adc.read(&mut self.adc_pin)?;
        Ok((raw as f32) * 3.3 / 4095.0)
    }

    fn read_temperature(&mut self) -> Result<f32> {
        let mut bmp = BMP280::new(self.i2c.clone(), 0x76);
        Ok(bmp.read_temperature()?)
    }

    fn read_light(&mut self) -> Result<f32> {
        let mut bh = BH1750::new(self.i2c.clone(), Resolution::High);
        Ok(bh.illuminance()?)
    }

    fn read_distance(&mut self) -> Result<f32> {
        self.trig_pin.set_low()?;
        FreeRtos::delay_ms(50);

        self.trig_pin.set_high()?;
        FreeRtos::delay_us(10);
        self.trig_pin.set_low()?;

        while !self.echo_pin.is_high() {}
        let start = unsafe { esp_idf_sys::esp_timer_get_time() };

        while self.echo_pin.is_high() {}
        let end = unsafe { esp_idf_sys::esp_timer_get_time() };

        let duration = (end - start) as f32;
        Ok((duration * 0.0343) / 2.0)
    }
}

#[entry]
fn main() -> ! {
    esp_idf_sys::link_patches();
    
    let peripherals = Peripherals::take().unwrap();
    let mut sensors = Sensors::new(peripherals).unwrap();
    
    let mut nvs = server_init::init_nvs();
    let wifi_config = server_init::load_config(&mut nvs);
    let wifi = init_wifi(peripherals.modem, wifi_config).unwrap();
    
    let mut server = if wifi.is_ap() {
        Some(server_init::start_webserver().unwrap())
    } else {
        None
    };

    loop {
        let sensor_data = SensorData {
            voltage: sensors.read_voltage().unwrap(),
            temperature: sensors.read_temperature().unwrap(),
            light: sensors.read_light().unwrap(),
            distance: sensors.read_distance().unwrap(),
        };

        log::info!(
            "Voltage: {:.2}V, Temp: {:.1}C, Light: {:.0}lux, Distance: {:.1}cm",
            sensor_data.voltage,
            sensor_data.temperature,
            sensor_data.light,
            sensor_data.distance
        );

       //TODO: sync with website and data sending

        FreeRtos::delay_ms(1000);
    }
}
