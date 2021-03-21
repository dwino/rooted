pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;
#[derive(Clone, Debug, PartialEq)]
pub struct PatrollingRandomly {
    pub path: Option<Vec<usize>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpawningFruit {
    pub template: Template,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpawningEquipment {
    pub template: Template,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RatAi {}
#[derive(Clone, Debug, PartialEq)]
pub struct AntAi {}
