use macroquad::prelude::*;
///Control State enum that's used to determine the flow of player input - decides what key does what when
#[derive(Clone, Debug, PartialEq)]
pub enum CombatActionType {
    ///Root state from which the user selects what actions they want to take that turn
    None,
    Movement,
    EndTurn,
}
