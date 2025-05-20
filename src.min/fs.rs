use esp_idf_svc::vfs::spiffs::Spiffs;
use esp_idf_svc::hal::prelude::*;

fn mount_fs() -> Result<Spiffs<'static>, EspError> {
    let config = esp_idf_svc::vfs::spiffs::Configuration::new()
        .partition("storage")  
        .format_on_error(true);
    
    Spiffs::new(&config)
}
