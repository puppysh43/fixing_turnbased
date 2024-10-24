mod prelude {
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;
    //fullscreen size is 60 tiles wide and 33.75 tiles tall
    //max screen size of 60/33 tiles with 12 pixels at the top and bottom as a buffer (or 24 at the top/bottom of the screen for text)
    pub const MAP_WIDTH: i32 = 45;
    pub const MAP_HEIGHT: i32 = 32;
    pub use std::collections::HashMap;
    pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
    pub use crate::components::*;
    pub use crate::gamestate::*;
}
mod combat_action_type;
mod combat_systems;
mod components;
mod gamestate;
mod init_ecs;
mod lib;
mod map;
mod sound_atlas;
mod texture_atlas;

use gamestate::*;
use macroquad::prelude::*;

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: macroquad::prelude::Conf {
            window_title: String::from("Combat Sandbox"),
            window_width: 1920,
            window_height: 1080,
            high_dpi: false,
            fullscreen: true,
            sample_count: 1,
            window_resizable: false,
            icon: None,
            ..Default::default()
        },
        update_on: Some(macroquad::conf::UpdateTrigger::default()),
        default_filter_mode: FilterMode::Nearest,
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //do all the on gamelaunch setup here
    let mut gamestate = GameState::default().await;

    //this is the actual gameloop for right now
    loop {
        //run all the game systems
        combat_systems::run(&mut gamestate);
        //quit if the player wants to quit
        if gamestate.quitting {
            break;
        }
        next_frame().await;
    }
}
