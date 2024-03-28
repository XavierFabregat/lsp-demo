use std::{error::Error, fs::File};

use simplelog::{self, format_description, ConfigBuilder};

pub fn get_logger(filename: &str) -> Result<(), Box<dyn Error>> {
    let logfile = File::create(filename)?;

    // configure Logger
    if let Err(e) = simplelog::WriteLogger::init(
        log::LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_format_custom(format_description!("[hour]:[minute]:[second].[subsecond]"))
            .set_location_level(log::LevelFilter::Error)
            .set_target_level(log::LevelFilter::Off)
            .set_thread_level(log::LevelFilter::Off)
            .build(),
        logfile,
    ) {
        return Err(Box::new(e));
    }
    Ok(())
}
