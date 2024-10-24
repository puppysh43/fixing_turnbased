use macroquad::prelude::*;

///Control State enum that's used to determine the flow of player input - decides what key does what when
#[derive(Clone, Debug, PartialEq)]
pub enum CombatActionType {
    ///Root state from which the user selects what actions they want to take that turn
    None,
    ///user wants to make a ranged attack during which they can move the reticule and select
    ///what kind of ranged attack they want to make!
    RangedAttack,
    ///user wants to make a melee attack, during which they can select a different kind of attack
    ///(I think just targetting body parts but that will be implemented much later)
    MeleeAttack,
    ///User wants to make a leadership check to apply bonuses/penalties to allies of their choice
    Leadership,
    ///User would like to target another character and Aim at them. [moving reticule and then pressing enter]
    Aiming,
    ///User can use up or down arrow to select which direction they want to change their stance
    ChangingStance,
    ///User will open up a menu about if they want to draw/holster and what item
    Drawing,
    ///user will reload their current weapon if applicable otherwise it will do nothing
    // Reloading,//may not actually need this one
    ///user will be able to move the character with the cursor keys for a limited amount of moves
    Movement,
    ///if user is not currently grappling they will be able to choose a direction [if needed] of who to try and grapple
    ///if the user is currently in a grapple it will instead bring up a menu of different grapple options (IMPLEMENT THIS MUCH LATER)
    Grapple,
    ///Allows the player to interact w/ an interactable object like in the overworld by automatically selecting any adjacent interactable object
    ///OR if needed specifying a direction
    Interact,
    ///todo later
    UseItem(UseItemState),
    ///todo later
    Look,
    ///todo later
    PickUp,
    ///Control for when a player wants to end their turn.
    EndTurn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UseItemState {
    Selecting,
    Using,
}
