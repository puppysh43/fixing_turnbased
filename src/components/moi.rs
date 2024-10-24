//this file will contain all of the messages of intent used by the game
use hecs::*;
use macroquad::prelude::*;
#[derive(Copy, Clone, Debug)]
pub struct MOIWantsToMove {
    collision: bool,
    entity: Entity,
    destination: IVec2,
}
impl MOIWantsToMove {
    pub fn new(collision: bool, entity: Entity, destination: IVec2) -> Self {
        Self {
            collision,
            entity,
            destination,
        }
    }
    pub fn get(&self) -> (bool, Entity, IVec2) {
        (self.collision, self.entity, self.destination)
    }
}
///Simple message component for communicating that the current character wants to end their turn
pub struct MOIEndTurn;
