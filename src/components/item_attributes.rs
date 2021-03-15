pub use crate::prelude::*;

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
