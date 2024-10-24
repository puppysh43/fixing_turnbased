use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;
pub fn system(state: &mut GameState) {
    debug_log(state);
    game_log(state);
}
fn debug_log(state: &mut GameState) {
    //make a query that goes through all debug log messages and for now just
    //prints them out with println
    let mut cmd_buf = CommandBuffer::new();
    for (id, debug_message) in state.ecs.query_mut::<&DebugLogMessage>() {
        println!("{}", debug_message.0.clone());
        cmd_buf.despawn(id);
    }
    cmd_buf.run_on(&mut state.ecs);
}
fn game_log(state: &mut GameState) {
    let mut cmd_buf = CommandBuffer::new();
    for (id, log_message) in state.ecs.query_mut::<&GameLogMessage>() {
        println!("{}", log_message.0.clone());
        cmd_buf.despawn(id);
    }
    cmd_buf.run_on(&mut state.ecs);
}
