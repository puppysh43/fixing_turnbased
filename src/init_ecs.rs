use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

///RN just have this setup for a basic fight but in the future this can be passed an identifier for deciding which
///scene to launch in the combat sandbox.
pub fn init_ecs() -> World {
    //first add the decorative entities (furniture, cover, whatever)
    //then spawn in the various characters in the combat scene
    //then define the combat scene
    //that should be everything??
    let mut ecs = World::new();
    let pc_01 = ecs.spawn((
        Renderable::new(String::from("character")),
        IVec2::new(0, 0),
        Attributes::default(),
        Skills::new(vec![
            (SkillType::RangedTwoHanded, 2),
            (SkillType::Ranged, 0),
            (SkillType::Melee, 0),
            (SkillType::Drive, 0),
        ]),
        ActionPoints::new(),
        MovementPoints::default(),
        Stance::Standing,
        ControlType::PC,
        Collideable,
    ));
    let npc_01 = ecs.spawn((
        Renderable::new(String::from("character")),
        IVec2::new(20, 20),
        Attributes::default(),
        Skills::new(vec![
            (SkillType::RangedTwoHanded, 2),
            (SkillType::Ranged, 0),
            (SkillType::Melee, 0),
            (SkillType::Drive, 0),
        ]),
        ActionPoints::new(),
        MovementPoints::default(),
        Stance::Standing,
        ControlType::NPC,
        Collideable,
    ));
    ecs.spawn((CombatEncounter::new(vec![pc_01, npc_01]),));
    ecs
}
