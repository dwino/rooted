pub use crate::prelude::*;
use std::collections::HashSet;

// GENERAL
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
#[derive(Clone, PartialEq)]
pub struct Name(pub String);

//ENTITIES

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Creature;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plant;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fruit;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

//PLAYER_AND_CREATURE_ATTRIBUTES

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

//ITEM_ATTRIBUTES

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);
#[derive(Clone, PartialEq)]
pub struct Equiped(pub Entity);
#[derive(Clone, PartialEq)]
pub struct Equipment();
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileStack(pub i32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangedDamage(pub i32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Armour;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Defense(pub i32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

//AI

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangedAttackingPlayer;
#[derive(Clone, Debug, PartialEq)]
pub struct SpawningFruit {
    pub template: Template,
}

//MESSAGES

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToRangedAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
