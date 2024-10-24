use std::collections::HashMap;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AttributeType {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Education,
    Charm,
}

///component that stores the attributes of an npc inspired by mongoose traveller 2e characteristics
pub struct Attributes {
    ///a character's natural physical strength
    strength: AttributeValue,
    ///a character's agility, reflexes, coordination, and fine motor control
    dexterity: AttributeValue,
    ///a character's physical stamina, determination, and ability to sustain damage
    endurance: AttributeValue,
    ///a character's raw intelligence and quickness of mind - used for new information
    ///and puzzle solving
    intelligence: AttributeValue,
    ///a character's level of lifetime learning and experience especially in academics/intellectual pursuits
    education: AttributeValue,
    ///a character's untrained charisma, social aptitude, and ability to relate to others
    charm: AttributeValue,
}
impl Attributes {
    pub fn new(
        strength: i32,
        dexterity: i32,
        endurance: i32,
        intelligence: i32,
        education: i32,
        charm: i32,
    ) -> Self {
        Self {
            strength: AttributeValue::new(strength),
            dexterity: AttributeValue::new(dexterity),
            endurance: AttributeValue::new(endurance),
            intelligence: AttributeValue::new(intelligence),
            education: AttributeValue::new(education),
            charm: AttributeValue::new(charm),
        }
    }
    pub fn default() -> Self {
        Self {
            strength: AttributeValue::new(7),
            dexterity: AttributeValue::new(7),
            endurance: AttributeValue::new(7),
            intelligence: AttributeValue::new(7),
            education: AttributeValue::new(7),
            charm: AttributeValue::new(7),
        }
    }
    ///provides an attribute given the attribute type enum to then have further data specified out of it
    pub fn attribute(&self, attribute: AttributeType) -> &AttributeValue {
        match attribute {
            AttributeType::Strength => &self.strength,
            AttributeType::Dexterity => &self.dexterity,
            AttributeType::Endurance => &self.endurance,
            AttributeType::Intelligence => &self.intelligence,
            AttributeType::Education => &self.education,
            AttributeType::Charm => &self.charm,
        }
    }
    ///provides mutable access to a specific attribute
    pub fn mut_attribute(&mut self, attribute: AttributeType) -> &AttributeValue {
        match attribute {
            AttributeType::Strength => &mut self.strength,
            AttributeType::Dexterity => &mut self.dexterity,
            AttributeType::Endurance => &mut self.endurance,
            AttributeType::Intelligence => &mut self.intelligence,
            AttributeType::Education => &mut self.education,
            AttributeType::Charm => &mut self.charm,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
///Holds all necessary data about the character's attributes
pub struct AttributeValue {
    current: i32,
    max: i32,
}
impl AttributeValue {
    pub fn new(value: i32) -> Self {
        Self {
            current: value,
            max: value,
        }
    }
    ///gets the current value of the given attribute. Used for almost all circumstances
    ///the game checks the attribute's value
    pub fn current(&self) -> i32 {
        self.current
    }
    ///gets the maximum value of the given attribute
    pub fn max(&self) -> i32 {
        self.max
    }
    ///Get the current bonus of the attribute used for almost all skill checks
    pub fn bonus(&self) -> i32 {
        match self.current {
            ..=0 => -3,
            1..=2 => -2,
            3..=5 => -1,
            6..=8 => 0,
            9..=11 => 1,
            12..=14 => 2,
            15.. => 3,
        }
    }
    ///Heal the attribute by a given delta, up to the maximum value of the attribute
    pub fn heal(&mut self, delta: i32) {
        if delta.is_positive() {
            if (self.current + delta) <= self.max {
                self.current += delta;
            } else {
                self.current = self.max;
            }
        }
    }
    ///Damage the attribute by a given delta
    pub fn damage(&mut self, delta: i32) {
        if delta.is_positive() {
            self.current -= delta;
        }
    }
}

///Component that holds all of the known skills a character has
pub struct Skills {
    known_skills: HashMap<SkillType, i32>,
}
impl Skills {
    ///Given a list of skills the character knows it'll turn it into the full component
    ///plus making sure all the needed base skills get added without needing to be made explicit
    pub fn new(skills: Vec<(SkillType, i32)>) -> Self {
        let mut known_skills: HashMap<SkillType, i32> = HashMap::new();
        for skill_tuple in skills.iter() {
            //if there's a base skill/general skill associated with the specialized skill they'll get that added too
            if Skills::get_base_skill(skill_tuple.0).is_some() {
                let base_skill = Skills::get_base_skill(skill_tuple.0).unwrap();
                known_skills.insert(base_skill, 0);
            }
            known_skills.insert(skill_tuple.0, skill_tuple.1);
        }
        Self { known_skills }
    }
    fn get_base_skill(skill: SkillType) -> Option<SkillType> {
        match skill {
            SkillType::MeleeUnarmed => Some(SkillType::Melee),
            SkillType::MeleeBlades => Some(SkillType::Melee),
            SkillType::MeleeBludgeoning => Some(SkillType::Melee),
            SkillType::RangedOneHanded => Some(SkillType::Ranged),
            SkillType::RangedTwoHanded => Some(SkillType::Ranged),
            SkillType::HeavyWeaponsArtillery => Some(SkillType::HeavyWeapons),
            SkillType::HeavyWeaponsPortable => Some(SkillType::HeavyWeapons),
            SkillType::HeavyWeaponsVehicle => Some(SkillType::HeavyWeapons),
            SkillType::AthleticsDexterity => Some(SkillType::Athletics),
            SkillType::AthleticsEndurance => Some(SkillType::Athletics),
            SkillType::AthleticsStrength => Some(SkillType::Athletics),
            SkillType::DriveWheels => Some(SkillType::Drive),
            SkillType::DriveWalker => Some(SkillType::Drive),
            SkillType::DriveTracked => Some(SkillType::Drive),
            _ => None,
        }
    }
    ///Used to get the dice modifier of the skill level a character has
    pub fn get_dm(&self, skill: SkillType) -> i32 {
        //First check if the player actually has the specific skill and if there are any associated base skills
        match (self.known_skills.get(&skill), Skills::get_base_skill(skill)) {
            //if the player has the specific skill being asked for then just return their skill level!
            (Some(skill_val), _) => *skill_val,
            //if they don't have the specific skill being asked for but the skill is one that's part of a general skill w/ specialties
            (None, Some(base_skill)) => {
                //then check if they have training in the base skill associated w/ the one yr checking for
                if self.known_skills.get(&base_skill).is_some() {
                    //and return 0 to represent a base level of competence
                    0
                } else {
                    //if it's a specialty in a general skill they have no training in then they have a DM of -3
                    -3
                }
            }
            //if they don't have the skill and it's not a skill w/ specialties just return the -3 DM
            (None, None) => -3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum SkillType {
    /*Combat Skills*/
    //primary combat skills
    Melee,
    MeleeUnarmed,
    MeleeBlades,
    MeleeBludgeoning,
    Ranged,
    RangedOneHanded,
    RangedTwoHanded,
    //secondary combat skills
    Explosives,
    HeavyWeapons,
    HeavyWeaponsArtillery,
    HeavyWeaponsPortable,
    HeavyWeaponsVehicle,
    /*Social Skills*/
    //primary social skills
    Broker,
    Persuade,
    Streetwise,
    //secondary social skills
    Deception,
    Leadership,
    Diplomat,
    /*Knowledge Skills*/
    //primary knowledge skills
    Electronics,
    Investigate,
    Mechanic,
    Medic,
    //secondary knowledge skills
    Admin,
    Advocate,
    Science,
    //possibly science subtypes
    LanguageBasic,
    LanguageBinaricCant,
    LanguageOuterAsh,
    /*Misc Skills*/
    //primary misc skills
    Athletics,
    AthleticsDexterity,
    AthleticsEndurance,
    AthleticsStrength,
    Stealth,
    Survival,
    Recon,
    //secondary misc skills
    AnimalHandling,
    Carouse,
    Drive,
    DriveWheels,
    DriveWalker,
    DriveTracked,
    Gambler,
    Navigation,
    VaccSuit,
}
