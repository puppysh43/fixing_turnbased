/*
This module contains all the helper functions for the game systems
*/
use crate::prelude::*;
use hecs::*;

pub fn calculate_initiative(state: &mut GameState) {
    //in the future this will take in arguments either what entities will be included or what mapscreen will have the combat encounter in it or WHATEVER
    //gay bullshit I end up doing
    //HOWEVER right now this simply looks for all entities that have the necessary components to even do an initiative test
}

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
