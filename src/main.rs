mod components;
mod events;
mod game;
mod map;
mod message;
mod ui;
mod utils;

use std::{error::Error, time::Duration};

use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(250))?;

    Ok(())
}
