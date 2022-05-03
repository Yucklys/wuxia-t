mod components;
mod events;
mod map;
mod message;
mod utils;

use std::{error::Error, time::Duration};

use assets_manager::AssetCache;
use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    let cache: AssetCache = AssetCache::new("assets").expect("Could not load ./assets folder");
    run(Duration::from_millis(250), &cache)?;

    Ok(())
}
