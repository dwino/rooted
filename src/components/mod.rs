pub use crate::prelude::*;

mod general;
pub use general::*;
mod entities;
pub use entities::*;
mod attributes;
pub use attributes::*;
mod item_attributes;
pub use item_attributes::*;

//AI

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;
#[derive(Clone, Debug, PartialEq)]
pub struct PatrollingRandomly {
    pub path: Option<Vec<usize>>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Foraging;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangedAttackingPlayer;
#[derive(Clone, Debug, PartialEq)]
pub struct SpawningFruit {
    pub template: Template,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpawningForager {}

//MESSAGES

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToDig {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMoveCamera {
    pub delta: Point,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsCycleTarget {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsEndInput(pub RlState);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsStateSwitch(pub EcoState);
