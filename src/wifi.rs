#![no_std]

use esp_idf_hal::{
    gpio::Pin,                     // Serial ports  
    adc::{Adc, Attenuation, ADC},  // For analogue sensors
    peripherals::Peripherals, 
    delay::Delay,     
};
use esp_wifi::{EspWifi, WiFiMode}; // Easier work with Wi-Fi
use esp_idf_sys::{
    gpio_install_isr_service,      //Isr handelrs 
    gpio_isr_handler_add,          //ISR reg
};

use esp_idf_sys::*;

#[derive(Deserealize)]
struct Wificonfig{
    SSID:     String, 
    Password: String, 
}

esp_log_level_set("*", ESP_LOG_INFO);

//FIXME
fn init_adc() -> Adc<'static, ADC> {
    let peripherals = unsafe { Peripherals::steal() };
    let adc = Adc::new(peripherals.adc1, &ADC::CONFIG_DEFAULT);
    adc.set_attenuation(Attenuation::Attenuation11dB); 
    adc
}

fn init_wifi() -> Result<(), EspError> {
    let mut wifi = EspWifi::new()?;
    wifi.set_mode(WiFiMode::STA)?;
    
    
    let mut sta_config = wifi::Config::default();
    sta_config.ssid = Wificonfig.SSID.into();       
    sta_config.password = Wificonfig.Password.into(); 
    
    wifi.set_configuration(&sta_config)?;
    wifi.start()?;
    wifi.connect()?; 

    if !wifi.is_connected()? {
        ESP_LOGI(TAG, "{} :Cannot connect wifi, retrying", gettimeofday());
        delay(Duration::from_secs(1));
        for(!wifi.is_connected; let i = 0; wifi <= 3){
            init_wifi();
            wifi++; 
            ESP_LOGI(TAG, "{} :Retrying connect, if this message is displayed less than three times, the connection is successful", gettimeofday());

        }
    }
    
    Ok(())
}


extern "C" fn gpio_isr_handler(arg: *mut ()) {
    
}

//FIXME: Porabally not valid  
fn init_interrupts() {
    unsafe {
        gpio_install_isr_service(0).unwrap(); 
        gpio_isr_handler_add(GPIO_NUM_4, gpio_isr_handler, core::ptr::null_mut());
    }
}

fn connect() -> ! {
    
    let peripherals = Peripherals::take().unwrap();
    let mut adc = init_adc();
    let mut sensor_pin = Pin::new(peripherals.pins.gpio34); 
    init_wifi().unwrap();
    init_interrupts();

    
    loop {
        let value = read_sensor(&mut adc, &mut sensor_pin);
        //TODO: Wifi data processing
    }
}

#![panic_handler]
fn panic(){

}
