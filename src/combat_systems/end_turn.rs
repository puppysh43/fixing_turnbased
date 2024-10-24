use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState, combat_encounter: &mut CombatEncounter) {
    //this will process requests to end the turn of the current active entity
    //create a bool to store if there is an moi
    let mut is_moi = false;
    let mut cmd_buffer = CommandBuffer::new();
    for (id, _moi) in state.ecs.query_mut::<&MOIEndTurn>() {
        //if there's an MOI pulled up by the query then mark the bool as such
        is_moi = true;
        cmd_buffer.despawn(id);
    }
    cmd_buffer.run_on(&mut state.ecs);
    //if there is a request to end the turn of the currently active entity
    //then do so
    if is_moi {
        state
            .ecs
            .spawn((DebugLogMessage::new(String::from("completing entity turn")),));
        let mut round_has_completed = false;
        let mut entities_in_combat: Vec<Entity> = Vec::new();
        for (_id, combat_encounter) in state.ecs.query_mut::<&mut CombatEncounter>() {
            round_has_completed = combat_encounter.complete_turn();
            entities_in_combat = combat_encounter.get_all_entities();
        }
        if round_has_completed {
            //if the round has completed then refresh the AP of all entities in the combat encounter
            for entity in entities_in_combat.iter() {
                for ap in state.ecs.query_one_mut::<&mut ActionPoints>(entity.clone()) {
                    ap.reset();
                }
            }
            //additionally in the future when a round has been completed an event will be sent into the ECS to be processed by any entity that
            //has a round limit, such as smoke from a smoke grenade
        }
    }
}
