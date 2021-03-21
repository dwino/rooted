pub use crate::prelude::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Creature;
//KIND
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Forager;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plant;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fruit;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MagicDroplet;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ForageSource;
