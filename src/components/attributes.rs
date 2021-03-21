pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    pub current: i32,
    pub max: i32,
}
pub struct Targeting {
    pub targets: Vec<(Entity, f32)>, // (entity / distance)
    pub current_target: Option<Entity>,
    pub index: usize,
}
pub struct Targetable;
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub sensing: bool,
    pub is_dirty: bool,
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            sensing: false,
            is_dirty: true,
        }
    }
    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            sensing: self.sensing,
            is_dirty: true,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TargetRange {
    pub reachable_tiles: HashSet<Point>,
    pub radius: i32,
}
impl TargetRange {
    pub fn new(radius: i32) -> Self {
        Self {
            reachable_tiles: HashSet::new(),
            radius,
        }
    }
}
