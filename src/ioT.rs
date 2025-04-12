
use anyhow::Result;
use embedded_svc::http::{client::Client, Method};
use esp_idf_hal::{
    adc::{Adc, AdcChannelDriver, AdcDriver, Attenuation, ADC1},
    gpio::AnyIOPin,
};
use esp_idf_sys::EspError;
use esp_idf_svc::http::client::{Configuration as HttpConfig, EspHttpClient};
use serde::Serialize;

pub struct IoTConfig {
    pub server_url: String,
    pub sampling_interval: u64,
}


pub struct IoTContext {
    adc: AdcDriver<'static, ADC1>,
    adc_pin: AdcChannelDriver<'static, { ADC1::CHANNEL0 }, Attenuation<11>>,
    config: IoTConfig,
    http_client: EspHttpClient,
}

#[derive(Serialize)]
struct SensorData {
    value: u16,
    sensor_type: &'static str,
}

impl IoTContext {
    pub fn new(
        adc_pin: AnyIOPin,
        config: IoTConfig,
    ) -> Result<Self, EspError> {
        // Инициализация ADC
        let adc = AdcDriver::new(ADC1::new()?)?;
        let adc_pin = AdcChannelDriver::new(adc_pin)?;

        // Инициализация HTTP-клиента
        let http_client = EspHttpClient::new(&HttpConfig {
            use_global_ca_store: true,
            ..Default::default()
        })?;

        Ok(Self {
            adc,
            adc_pin,
            config,
            http_client,
        })
    }

    pub fn read_analog(&mut self) -> Result<u16> {
        Ok(self.adc.read(&mut self.adc_pin)?)
    }

    pub fn send_to_server(&mut self, value: u16) -> Result<()> {
        let data = SensorData {
            value,
            sensor_type: "analog",
        };

        let json = serde_json::to_string(&data)?;
        
        let mut request = self.http_client.request(
            Method::Post,
            &self.config.server_url,
            &[("Content-Type", "application/json")]
        )?;
        
        request.write(json.as_bytes())?;
        request.flush()?;

        let status = request.status();
        if (200..300).contains(&status) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("HTTP Error: {}", status).into())
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let value = self.read_analog()?;
            self.send_to_server(value)?;
            

            esp_idf_hal::delay::FreeRtos::delay_ms(
                self.config.sampling_interval
            );
        }
    }
}