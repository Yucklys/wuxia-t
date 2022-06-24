mod components;
mod events;
mod game;
mod message;
mod ui;
mod utils;
mod world;

use std::{error::Error, time::Duration};

use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(250))?;

    Ok(())
}
