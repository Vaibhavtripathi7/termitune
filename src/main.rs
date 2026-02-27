use std::error::Error;
use log::info;

mod ui;
mod audio;
mod queue;
mod input;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting termitune");
    
    ui::run()?;
    
    Ok(())
}
