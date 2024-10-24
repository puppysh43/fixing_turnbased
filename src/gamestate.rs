use crate::combat_action_type::*;
use crate::init_ecs::*;
use crate::map::*;
use crate::prelude::*;
use crate::sound_atlas::*;
use crate::texture_atlas::*;
use hecs::*;
use macroquad::prelude::*;

pub struct GameState {
    pub texture_atlas: TextureAtlas,
    pub sound_atlas: SoundAtlas,
    pub ecs: World,
    pub turn_state: TurnState,
    pub control_state: CombatActionType,
    pub map: Map, //temporary just for testing combat.
    pub log: Vec<String>,
    pub number_turns: i32,
    pub quitting: bool,
    pub event_queue: Vec<GameEvent>, //ui_state: UiState,
}

impl GameState {
    pub async fn default() -> Self {
        let log: Vec<String> = Vec::new();
        let event_queue: Vec<GameEvent> = Vec::new();
        Self {
            texture_atlas: crate::texture_atlas::make().await,
            sound_atlas: crate::sound_atlas::make().await,
            ecs: crate::init_ecs::init_ecs(),
            turn_state: TurnState::PlayerOne,
            control_state: CombatActionType::None,
            map: Map::new(),
            log,
            number_turns: 0,
            quitting: false,
            event_queue,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TurnState {
    PlayerOne,
    PlayerTwo,
}
pub enum GameEvent {
    EntityMoved,
}
