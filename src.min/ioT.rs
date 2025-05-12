//TODO: Async work, не ну я че долбоеб все сразу делать 

#[no_std]
#[no_main]

// Main libs
use core::fmt;
use anyhow::Result;
use esp_idf_hal::{
    adc::{Adc, AdcChannelDriver, AdcDriver, Attenuantion, ADC1}
    gpio::AnyIOPin
};
use esp_idf_sys::prelude; 
use esp_idf_hal::{
    config::Config,
    peripherals::Peripherals,
    delay::FreeRtos
}; 
use nanoserde::{Serialize, Deserialize};

// My headers 
mod server-init 

use server-init::{
    Wifi::{init_wifi, wifi_status}
};

//Sensors 
use bmp280::BMP280;
use bh1750::{BH1750, Resolution};

impl Sersors { 

//After implementing many sensors supports i will fix this function, universaly and optimisation 
//my scare 
    fn initialise -> anyhow::Result<()>{
        let peripherals = Peripherals::take()?;
        let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;
        let mut pin = peripherals.pins.gpio.into_analog()?;

        loop {
            let value = adc.read(&mut pin)?;
            log_voltage_data(value);
            FreeRtos::delay_ms(1000);
            /* 
            
             Что-то вродe того должно получиться что если 
             узнаем о подключении сенсора мы его инициализируем, а 
             потом проверяем по интервалам в веб сервере, impl Actions для 
             того чтобы совершить действие в зависимости от значения сенсора.

             */
        }
    }
    fn log_voltage_data(raw_value: u16) {
        let voltage = (raw_value as f32) * 3.3 / 4095.0; //Че за пиздец
        esp_println::println!("Voltage: {:.2}v", voltage); //Ну пускай будет так для начала
        //TODO: Integration with webserver
        //FIXME: I think this dont work stable, recheck 
    }

    //Temperature
    fn bmp280_sensor -> anyhow::Result<()>{
        let 12c = I2cDriver::new(
            peripherals::i2c0,
            peripherals::pins::gpio21, 
            peripherals::pins::gpio22,                 //TODO: Fix the gpio, need to real-time sensors processing
            &I2cConfig::new().baudrate(100.kHz().into),//in any gpio port
        )?;

        let mut bmp280 = BMP280::new(i2c, 0x76);
        let temperature = bpm280.read_temperature()?;
        esp_println::println!("Temperature: {:.2} C", temperature);
    }

    fn bh1750_sensor -> anyhow::Result<()>{
        let i2c = I2C::new( //Stilled from docs 
            peripherals.I2C0,
            peripherals::pins::gpio8,
            peripherals::pins::gpio9,
            &I2cConfig::new().baudrate(100.kHz().into()),
            &clocks,
            None
    )?;
        let bh1750 = BH1750::new(i2c, delay, false)
        let lux = bh1750.get_one_time_measurement(Resolution::High)?;
        esp_println::println!("Light: {:.2} Lux", lux);
    }

    fn hcsr04_sensor() -> anyhow::Result<()> {
         let mut trig = PinDriver::output(peripherals.pins.gpio5)?;
         let mut echo = PinDriver::input(peripherals.pins.gpio6)?;

         trig.set_low()?;
         FreeRtos::delay_ms(50);

         loop {
             trig.set_high()?;
             FreeRtos::delay_us(10);
             trig.set_low()?;

             while !echo.is_high() {}

             let start = unsafe { esp::esp_timer_get_time() };
             
             while echo.is_high() {}

             let end = unsafe { esp::esp_timer_get_time() };

             let duration = end - start; 
             let distance = (duration as f32 *  0.0343) / 2.0;
             esp_println::println!("distance: {:.2} cm", distance);
             FreeRtos::delay_ms(1000);
         }
    }

}



impl ioT-main {

}

impl Actions {

}

