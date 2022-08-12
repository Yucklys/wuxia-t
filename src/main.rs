mod components;
mod game;
mod utils;

use std::{error::Error, time::Duration};

use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(250))?;

    Ok(())
}
