pub use crate::prelude::*;

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
