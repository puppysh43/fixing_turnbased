/*
This module contains all the helper functions for the game systems
*/
use crate::prelude::*;
use hecs::*;
///Reset the amount of movement points an entity has so they can move more than once ever.
pub fn refresh_mp(state: &mut GameState, active_entity: Entity) {
    //the way I'm doing it rn feels really hacky and lazy but whatever
    for mp in state
        .ecs
        .query_one_mut::<&mut MovementPoints>(active_entity)
    {
        mp.reset();
        println!("resetting movement points.");
    }
}
