use macroquad::prelude::*;
use std::collections::HashMap;

pub type TextureAtlas = HashMap<String, Texture2D>;

pub async fn make() -> TextureAtlas {
    set_pc_assets_folder("resources");
    let default_character = load_texture("textures/default_character.png")
        .await
        .expect("Failed to load texture.");
    let default_floor = load_texture("textures/default_floor.png")
        .await
        .expect("Failed to load texture.");
    let default_fullcover = load_texture("textures/default_fullcover.png")
        .await
        .expect("Failed to load texture.");
    let default_halfcover = load_texture("textures/default_halfcover.png")
        .await
        .expect("Failed to load texture.");
    let default_quartercover = load_texture("textures/default_quartercover.png")
        .await
        .expect("Failed to load texture.");
    let default_wall = load_texture("textures/default_wall.png")
        .await
        .expect("Failed to load texture.");
    let down_arrow = load_texture("textures/down_arrow.png")
        .await
        .expect("Failed to load texture.");
    let texture_atlas = HashMap::from([
        (String::from("character"), default_character),
        (String::from("floor"), default_floor),
        (String::from("full cover"), default_fullcover),
        (String::from("half cover"), default_halfcover),
        (String::from("quarter cover"), default_quartercover),
        (String::from("wall"), default_wall),
        (String::from("down arrow"), down_arrow),
    ]);
    build_textures_atlas();
    return texture_atlas;
}
