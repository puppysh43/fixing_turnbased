use macroquad::audio::*;
use macroquad::prelude::*;
use std::collections::HashMap;

pub type SoundAtlas = HashMap<String, Sound>;

pub async fn make() -> SoundAtlas {
    set_pc_assets_folder("resources");
    let chime = load_sound("sounds/chime.wav").await.unwrap();
    let sound_atlas = HashMap::from([(String::from("chime"), chime)]);
    return sound_atlas;
}
