mod config;
mod define;
mod game;

use bevy::prelude::*;

use game::plugin::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
