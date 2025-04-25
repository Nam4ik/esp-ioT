// sensor.rs
use esp_idf_hal::{
    adc::{Adc, Adtenuation, AdcChannelDriver, ADC1},
    gpio::AnyIOPin,
    delay::FreeRtos,
    peripherals::Peripherals,
};
use anyhow::{Result, bail};

pub struct TemperatureSensor {
    adc: Adc<ADC1>,
    channel: AdcChannelDriver<'static, ADC1>,
}

impl TemperatureSensor {
    pub fn new(pin: AnyIOPin) -> Result<Self> {
        let peripherals = Peripherals::take().unwrap();
        let mut adc = Adc::new(peripherals.adc1)?;
        let mut channel = AdcChannelDriver::new(pin)?;
        channel.set_attenuation(Attenuation::Db11);
        
        Ok(Self { adc, channel })
    }

    pub fn read_celsius(&mut self) -> Result<f32> {
        const V_REF: f32 = 3.3;
        const MAX_ADC: f32 = 4095.0;
        
        let raw = self.adc.read(&mut self.channel)?;
        let voltage = (raw as f32) * V_REF / MAX_ADC;
        
        // Пример для термистора NTC 10K
        let temp_k = 1.0 / (0.001129148 + (0.000234125 * voltage) + (0.0000000876741 * voltage.powf(2.0)));
        Ok(temp_k - 273.15)
    }
}
