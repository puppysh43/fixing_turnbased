mod end_turn;
mod input;
mod logs;
mod movement;
mod render;

use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;
//this file is going to run all of the combat systems
//the goal is to make this AS PORTABLE AS POSSIBLE so it can be seemlessly integrated
//into the full version of the rpg with MINIMAL EFFORT
pub fn run(state: &mut GameState) {
    //run all the combat systems here
    //first I guess get the current combat encounter
    let mut combat_encounter = get_combat_encounter(state).expect("No combat encounter in ECS!");
    //MVP for testing proof of concept
    //it will allow the user to have a field of characters that it will alternate between and allow JUST movement
    //printing to the console how many action points are left, what round it is, who's turn it is, etc
    //first get the player's input and get MOIs put into the ecs
    input::system(state, &mut combat_encounter);
    //then process those MOIs and do other systems
    movement::system(state);
    end_turn::system(state, &mut combat_encounter);
    //then render the gamestate onto the screen
    logs::system(state);
    render::system(state);
    update_combat_encounter(state, combat_encounter);
}
///Helper function that returns the current Combat Encounter in the ECS
fn get_combat_encounter(state: &mut GameState) -> Option<CombatEncounter> {
    for (_id, combat_encounter) in state.ecs.query_mut::<&CombatEncounter>() {
        return Some(combat_encounter.clone());
    }
    return None;
}

///Helper function that updates the combat encounter in the ECS w/ the one that you've been mutating
///in yr game systems outside the World
fn update_combat_encounter(state: &mut GameState, combat_encounter: CombatEncounter) {
    //make a command buffer to make the borrow checker happy
    let mut command_buffer = CommandBuffer::new();
    //iterate through all combat encounters in the World and mark them for despawning (there should only ever be one in the World at once)
    for (id, _combat_encounter) in state.ecs.query::<&CombatEncounter>().iter() {
        command_buffer.despawn(id);
    }
    //run the command buffer of despawns on the world
    command_buffer.run_on(&mut state.ecs);
    //spawn in the mutated combat encounter struct passed as an argument into the World
    state.ecs.spawn((combat_encounter,));
}
