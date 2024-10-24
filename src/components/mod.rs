mod moi;
pub use moi::*;

use hecs::*;
use macroquad::prelude::*;
//this is the big mod file for components that will expand as I add more components

///Component that allows an entity to be rendered, contains the hashmap key needed to retrieve
///the necessary Texture2D from the texture atlas
#[derive(Clone, Debug)]
pub struct Renderable {
    sprite: String,
}
impl Renderable {
    pub fn new(sprite: String) -> Self {
        Self { sprite }
    }
    pub fn get_sprite(&self) -> String {
        self.sprite.clone()
    }
}
///Newtype wrapper around a 2D vector integer used for tracking the location of tile-map entities on the screen
///(this will be basically everything except for effects)
// pub struct GridPosition(IVec2);

///This enum will be used in this sandbox simply for determining which team someone is on but in the future
///will be used for determining who in combat is controlled by the player and who by AI
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ControlType {
    PC,  //for hotseat mode synonymous w/ player 1
    NPC, //for hotseat mode synonymous w/ player 2
}

///Struct that defines who is in a combat encounter, and tracks all the information needed to
///process through them in iniatiative order
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CombatEncounter {
    ///The actual order of the vec is what determines their initiative order, tracked by the character's
    ///Entity ID tag. it's paired with a boolean to track who has completed their turn
    initiative_order: Vec<(Entity, bool)>,
    ///The total number of turns the combat encounter took is tracked and recorded
    num_rounds: i32,
}
impl CombatEncounter {
    ///Creates a new Combat Encounter. The Vec of Entities passed to it HAS to already be sorted by initiative order in a separate function.
    pub fn new(combatants: Vec<Entity>) -> Self {
        let mut initiative_order: Vec<(Entity, bool)> = Vec::new();
        for entity in combatants.iter() {
            initiative_order.push((*entity, false));
        }
        Self {
            initiative_order,
            num_rounds: 0,
        }
    }

    ///Returns the entity ID of the next character in the initiative order to go.
    pub fn next_turn(&self) -> Option<Entity> {
        //iterate through all the entities in the combat encounter and return the entity id of the first one that hasn't been marked
        //as having completed their turn that round
        for (entity, completed_turn) in self.initiative_order.iter() {
            if !completed_turn {
                return Some(*entity);
            }
        }
        return None;
    }
    ///Used to mark a character's turn in the initiative order as complete. By default done whenever a character uses up
    ///all of their Action Points. Returns a bool that expresses whether the turn being completed also completes a full round
    ///expressing to other game systems that stuff like action points needs to be refreshed and other misc systems
    ///(eg. poison gas that lingers for 3 rounds)
    pub fn complete_turn(&mut self) -> bool {
        //iterate through the initiative order
        for (entity, completed_turn) in self.initiative_order.iter_mut() {
            //when you find the first one that hasn't completed their turn mark it as complete
            if !*completed_turn {
                *completed_turn = true;
                //break out of the loop so that only one is marked as complete!
                break;
            }
        }

        //finally check to see if every entity in the initiative order has completed their turn and if so
        //reset every entity in the initiative order to having not completed
        self.check_round_completion()
    }
    ///Used to complete a round, ticking up the number of rounds and resetting every entity in the initiative order to being marked
    ///as having not gone yet. Returns a bool to express if the round has been completed or not.
    pub fn check_round_completion(&mut self) -> bool {
        //flag for completion automatically set to true
        let mut is_complete = true;
        //iterate through the initiative order to make sure everyone has completed their turn.
        for (_, completed_turn) in self.initiative_order.iter() {
            //if any of them haven't completed their turn yet then mark the round as NOT completed
            if !completed_turn {
                is_complete = false;
            }
        }
        //if everyone in the iniative order has taken their turn then increment the round count and reset the initiative order
        //back to the beginning where no one has gone yet.
        if is_complete {
            self.num_rounds += 1;
            for (_entity, completed_turn) in self.initiative_order.iter_mut() {
                *completed_turn = false;
            }
            return true;
        } else {
            return false;
        }
    }
    ///returns all of the entities in the combat encounter in initiative order.
    pub fn get_all_entities(&self) -> Vec<Entity> {
        let mut all_entities: Vec<Entity> = Vec::new();
        for (entity_id, _has_completed) in self.initiative_order.iter() {
            all_entities.push(entity_id.clone());
        }
        all_entities
    }
}

///An entity's action points used for the turn based combat system
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ActionPoints(i32);
impl ActionPoints {
    pub fn new() -> Self {
        Self(3)
    }
    ///Reduces the action points by the 2AP that a significant action costs if possible
    ///otherwise just returns the current amount of action points
    pub fn significant_action(&mut self) -> Result<i32, i32> {
        if self.0 >= 2 {
            self.0 -= 2;
            Ok(self.0)
        } else {
            Err(self.0)
        }
    }
    ///Reduces the action points by the 1AP that a minor action costs
    pub fn minor_action(&mut self) -> Result<i32, i32> {
        if self.0 >= 1 {
            self.0 -= 1;
            Ok(self.0)
        } else {
            Err(self.0)
        }
    }
    ///consumes all of the character's action points at once
    pub fn full_turn(&mut self) {
        self.0 -= 3;
    }
    ///Resets the Action Points back to the start/default
    pub fn reset(&mut self) {
        self.0 = 3;
    }
    ///provides how many action points are currently left
    pub fn get(&self) -> i32 {
        self.0
    }
}

///tag component that points to a weapon and marks it as equipped by an entity that has this component
pub struct EquippedRangedWeapon(Entity);

///Component assigned to an entity when it's in the moving phase,
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MovementPoints {
    max: i32,
    current: i32,
}
impl MovementPoints {
    ///create a custom amount of movement points for non-standard enemies
    pub fn new(max: i32) -> Self {
        Self { max, current: max }
    }
    ///create the movement point component for a standard humanoid character (4 tiles or 6 meters)
    pub fn default() -> Self {
        Self { max: 3, current: 3 }
    }
    ///reduces the movement points by one. used every time an entity moves. an entity will never
    ///consume more than one movement point at a time
    pub fn decrement(&mut self) {
        self.current -= 1;
    }
    ///reset the current movement points (done at the beginning of an entity initiating their action)
    pub fn reset(&mut self) {
        self.current = self.max;
    }
    ///checks if there's any movement points left
    pub fn can_move(&self) -> bool {
        if self.current > 0 {
            true
        } else {
            false
        }
    }
    ///returns the current amount of movement points
    pub fn current(&self) -> i32 {
        self.current
    }
}
///Component spawned to add a string to debug log used to track game/engine behavior
///that can be displayed if a flag is set
//this structure allows us to be agnostic about how these messages are displayed or handled in the engine
pub struct DebugLogMessage(pub String);

impl DebugLogMessage {
    pub fn new(contents: String) -> Self {
        Self(contents)
    }
}
///Component spawned to add a string to the game log that will be displayed in the UI
//this structure allows us to handle the game log different ways as the engine evolves
pub struct GameLogMessage(pub String);

impl GameLogMessage {
    pub fn new(contents: String) -> Self {
        Self(contents)
    }
}
