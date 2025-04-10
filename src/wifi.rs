#![no_std]

use esp_idf_hal::{
    gpio::{Pin, Input, Output},  // Для управления пинами
    adc::{Adc, Attenuation},     // Для аналоговых датчиков
    i2c::I2c,                    // Для датчиков с интерфейсом I2C
    peripherals::Peripherals,     // Инициализация периферии
};

/*
use esp_idf_sys::{
    esp_wifi_init,               // Инициализация Wi-Fi
    esp_wifi_set_mode,           // Настройка режима (STA/AP)
    esp_wifi_start,              // Запуск Wi-Fi
};
use esp_wifi::{wifi, wifi::*, EspWifi}; // Упрощённая работа с Wi-Fi
                    
use embassy_executor::Executor; // Асинхронный executor

use esp_idf_sys::{
    gpio_install_isr_service,    // Установка обработчика прерываний
    gpio_isr_handler_add,        // Регистрация ISR
};
*/

use serde::{Serialize, Deserealize};

use core::fmt::error;

#![no_std]

use esp_idf_hal::{
    gpio::{Pin, Input, Output, Pull},
    adc::{Adc, Attenuation, ADC},
    peripherals::Peripherals,
};
use esp_idf_sys::*;
use embassy_executor::Executor;
use core::fmt::Write; 


fn init_adc() -> Adc<'static, ADC> {
    let peripherals = unsafe { Peripherals::steal() };
    let adc = Adc::new(peripherals.adc1, &ADC::CONFIG_DEFAULT);
    adc.set_attenuation(Attenuation::Attenuation11dB); // Расширяет диапазон до 0-3.3В [[6]]
    adc
}

fn init_wifi() -> Result<(), EspError> {
    let mut wifi = EspWifi::new()?;
    wifi.set_mode(WiFiMode::STA)?;
    wifi.start()?;
    Ok(())
}

extern "C" fn gpio_isr_handler(arg: *mut ()) {
    
}


fn init_interrupts() {
    unsafe {
        gpio_install_isr_service(0).unwrap(); // Инициализация сервиса [[8]]
        gpio_isr_handler_add(GPIO_NUM_4, gpio_isr_handler, core::ptr::null_mut());
    }
}

fn connect() -> ! {
    // Инициализация
    let peripherals = Peripherals::take().unwrap();
    let mut adc = init_adc();
    let mut sensor_pin = Pin::new(peripherals.pins.gpio34); // Пример GPIO для ADC [[6]]
    init_wifi().unwrap();
    init_interrupts();

    // Основной цикл
    loop {
        let value = read_sensor(&mut adc, &mut sensor_pin);
        // Обработка данных
    }
}