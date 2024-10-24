use crate::map::*;
use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState) {
    //first render the gamemap
    render_map(state);
    //then render the entities in proper order (the z ordering equivalent is just that they're drawn in the order of the draw_texture function)
    render_entities(state);
}
fn render_map(state: &mut GameState) {
    clear_background(GRAY);
    //first render the game map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pt = IVec2::new(x, y);
            let idx = map_idx(x, y);
            if state.map.in_bounds(pt) {
                match state.map.tiles[idx] {
                    TileType::Wall => {
                        draw_texture(
                            state.texture_atlas.get("wall").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::Floor => {
                        draw_texture(
                            state.texture_atlas.get("floor").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                }
            }
        }
    }
}

fn render_entities(state: &mut GameState) {
    for (_id, (sprite_id, pos, ctrl_type)) in
        state.ecs.query_mut::<(&Renderable, &IVec2, &ControlType)>()
    {
        let mut color = Color::new(0.0, 0.0, 0.0, 0.0);
        match ctrl_type {
            ControlType::PC => color = RED,
            ControlType::NPC => color = BLUE,
        }
        draw_texture(
            state.texture_atlas.get(&sprite_id.get_sprite()).unwrap(),
            (pos.x * TILE_WIDTH) as f32,
            (pos.y * TILE_HEIGHT) as f32,
            color,
        );
    }
}
