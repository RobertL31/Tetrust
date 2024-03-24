
use lazy_static::lazy_static;
use std::collections::HashMap;

use config::{Config, ConfigError};

use crate::gameboard::{Action, Direction};


lazy_static! {
    pub static ref CONFIG_INPUT_LABELS : HashMap<Action, &'static str> = HashMap::from([
        (Action::Move(Direction::Left), "left"),
        (Action::Move(Direction::Top), "hard_drop"),
        (Action::Move(Direction::Right), "right"),
        (Action::Move(Direction::Bottom), "soft_drop"),
        (Action::Rotate, "rotate"),
        (Action::Hold, "hold"),
    ]);
}

static CONFIG_FILENAMES: &[&str] = &[
    &"config/inputs.json"
];


pub fn get_config() -> Result<Config, ConfigError>{

    let mut config_builder = Config::builder();

    for config_file in CONFIG_FILENAMES {
        config_builder = config_builder.add_source(config::File::with_name(&config_file));
    }
    
    config_builder.build()
}