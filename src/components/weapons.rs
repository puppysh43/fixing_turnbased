///A collection of misc. effects that can be given to weapons to represent various features
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WeaponTrait {
    ///Armor Piercing and will ignore the amount of protection equal to the contained integer value
    AP(i32),
    ///to be implemented later
    Artillery,
    ///Capable of automatic fire giving the weapon access to 3 different fire modes: single, burst, and full auto
    ///single attacks are normal, burst attacks get the contained int value added to the damage and use the int value of rounds
    ///full auto attacks can make a number of attacks equal to the contained int value against any targets w/in 4 tiles of each other
    ///full auto uses a number of rounds equal to 3x the contained int value
    Auto(i32),
    ///Has an explosive or AoE component. On a successful attack damage is rolled against every target w/in a circle
    ///w/ the radius of the contained int value (in meters or tiles undecided). Targeted enemies can't make a a dodge
    ///reaction but can dive for cover. cover is calculated from the center of the blast not the tile of the attacker
    ///NEED TO DECIDE HOW ATTACK ROLLS FOR THIS ARE IMPLEMENTED will probably be a late addition
    Blast(i32),
    ///Has powerful recoil or is just extremely heavy and requires STR 9 or higher to use w/out penalty
    ///if the user doesn't have enough STR the attack rolls will have a negative DM equal to the difference
    ///between their STR DM and +1
    ///Alternatively just check for the DM and if it's less than 1 get the difference
    Bulky,
    ///Has some form of magnified optic, allowing the weapon to ignore standard limits on range that make
    ///all attacks made at over 100 meters (67 tiles) be counted as at Extreme Range as long as the user
    ///aims before shooting
    Scope,
    ///Designed to deal non-lethal damage, incapitating rather than killing. Damage is only deducted from END,
    ///reduced by protection as normal. If the target's END is reduced to 0 by a stun weapon the target will be
    ///unable to do any actions for a num of rounds equal to how much the dmg exceeded their END(or how negative their DM is)
    ///Damage dealt by stun weapons is completely healed by one hour of rest.
    Stun,
    ///Extremely heavy or very intense recoil. It requires STR 12/an STR DM of +2 to use w/out penalty
    ///if not all attack rolls have a negative DM to the attack rolls equal to the difference between their STR DM and +2
    VeryBulky,
    ///Whether through poor quality or inherent flaws the weapon can be as lethal to the user as the target
    ///If an attack roll is made w/ the weapon w/ Effect -5 or worse it explodes inflicting the damage roll on the user
    ///and not the target, and then the weapon is broken beyond use.
    Dangerous,
    ///Sets the target on fire, causing dmg every round after the initial attack. A target can only be on fire from one weapon
    ///at once using the highest damage fire weapon. On its own a fire will extinguish itself on a 2D roll of 8+, rolled at the
    ///start of every round. A character may use 2 Action Points to extinguish the fire requiring an avg (8+) DEX check.
    ///the character gains DM+2 if using firefighting equipment. WILL BE IMPLEMENTED MUCH LATER IF AT ALL
    Fire,
    ///Is designed to be disposable. After being used once the weapon will be rendered inoperable
    OneUse,
    ///Either through compensating for weapon noise or never making it at all the weapon is functionally silent.
    ///Any attempts to detect the source of this sound suffer DM-6
    Silent,
    ///Is particularly heavy and has immense momentum when swung. A character attacked by a weapon w/ the Smasher trait
    ///may not attempt to parry it when reacting to the attack.
    Smasher,
    ///Extremely unstable. If an attack roll is made with Effect -3 or worse it explodes and the damage roll is inflicted
    ///upon the person firing it and the weapon is rendered inoperable
    VeryDangerous,
    ///This weapon isn't designed for harm but is instead repurposed in the heat of battle. It otherwise resembles an existing weapon
    ///but has DM-1 to attack rolls and -1 to each damage dice. After any attack roll w/ an Effect of -3 or less it breaks permanently.
    Improvised,
    ///Has been finely tuned for greater precision. It gets DM+1 to attack rolls
    Accurate,
    ///Has been tweaked for greater than usual range. The range of this weapon is increased by 50% [unsure if I'll implement this one]
    LongRange,
}

///Enum for identifying what skill is used for the attack roll of a weapon
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WeaponType {
    MeleeUnarmed,
    MeleeBlades,
    MeleeBludgeoning,
    RangedOneHanded,
    RangedTwoHanded,
    HeavyWeaponsArtillery,
    HeavyWeaponsPortable,
    HeavyWeaponsVehicle,
    Thrown, //for thrown weapons Athletics(dexterity) is used
}

pub enum WeaponSize {
    OneHanded,
    TwoHanded,
}

///Contains all the data necessary for a weapon to be interacted with
pub struct Weapon {
    ///attack type used for determining both rules interactions and what animations will play
    attack_type: WeaponType,
    ///how many hands a weapon uses
    size: WeaponSize,
    ///range is in meters
    range: i32,
    ///damage stored as a string that will can be parsed into a damage roll
    damage: String,
    ///weight in kilograms
    kg: f32,
    ///cost in credits
    cost: i32,
    ///magazine size represented as an i32. note that current ammo will be stored as a separate component
    magazine: i32,
    //magazine_cost: i32,
}

///Component for when a weapon is loaded with ammo
pub struct CurrentAmmo(i32);
impl CurrentAmmo {}
