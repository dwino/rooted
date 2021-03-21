use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Equipment,
    Plant,
    Fruit,
    Creature,
}
#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]

pub enum AiType {
    MovingRandomly,
    Chasing,
    RangedAttacking,
    RatAi,
    AntAi,
}
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum FruitType {
    Healing,
    Knowing,
    Sensing,
}
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum EquipmentType {
    Ranged,
    Melee,
    Armour,
    RangedPlus,
    MeleePlus,
    ArmourPlus,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub color: String,
    pub ai_type: Option<AiType>,
    pub fruit_type: Option<FruitType>,
    pub equipment_type: Option<EquipmentType>,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
    pub base_ranged_damage: Option<i32>,
    pub base_defense: Option<i32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
    pub fruits: HashMap<FruitType, Template>,
    pub equipments: HashMap<EquipmentType, Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();

        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut commands = legion::systems::CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(*pt, entity, &mut commands, rng);
            }
        });
        commands.flush(ecs);
    }
    fn spawn_entity(
        &self,
        pt: Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer,
        rng: &mut RandomNumberGenerator,
    ) {
        let color_string = RGB::from_hex(template.color.clone()).expect("Bad Hex");
        let entity = commands.push((
            pt,
            Render {
                color: ColorPair::new(color_string, RGB::from_hex("#D7E7D0").unwrap()),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));
        match template.entity_type {
            EntityType::Equipment => {
                commands.add_component(entity, Item {});
                commands.add_component(entity, Equipment {});
            }
            EntityType::Fruit => commands.add_component(entity, Item {}),
            EntityType::Plant => {
                commands.add_component(entity, Plant {});
                if let Some(fruit_type) = template.fruit_type {
                    commands.add_component(
                        entity,
                        SpawningFruit {
                            template: self.fruits.get(&fruit_type).unwrap().clone(),
                        },
                    );
                }
                if let Some(equipment_type) = template.equipment_type {
                    commands.add_component(
                        entity,
                        SpawningEquipment {
                            template: self.equipments.get(&equipment_type).unwrap().clone(),
                        },
                    );
                }
            }
            EntityType::Creature => {
                commands.add_component(entity, Creature {});
                commands.add_component(entity, FieldOfView::new(7));
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
                commands.add_component(entity, Targetable {});

                match template.ai_type.unwrap() {
                    AiType::MovingRandomly => {
                        commands.add_component(entity, MovingRandomly {});
                    }
                    AiType::Chasing => {
                        commands.add_component(entity, ChasingPlayer {});
                    }
                    AiType::RangedAttacking => {
                        commands.add_component(entity, RangedAttackingPlayer {});
                    }
                    AiType::RatAi => {
                        commands.add_component(entity, RatAi {});
                        commands.add_component(
                            entity,
                            PatrollingRandomly {
                                path: Some(Vec::new()),
                            },
                        );
                        commands.add_component(
                            entity,
                            Energy {
                                current: rng.range(10, 25),
                                max: 50,
                            },
                        );
                    }
                    AiType::AntAi => {
                        commands.add_component(entity, AntAi {});
                        commands.add_component(
                            entity,
                            PatrollingRandomly {
                                path: Some(Vec::new()),
                            },
                        );
                        commands.add_component(
                            entity,
                            Energy {
                                current: rng.range(10, 25),
                                max: 50,
                            },
                        );
                    }
                }
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "Sensing" => commands.add_component(entity, ProvidesSensing { amount: *n }),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides);
                    }
                });
        }
        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Equipment {
                commands.add_component(entity, Weapon {});
            }
        }
        if let Some(defense) = &template.base_defense {
            commands.add_component(entity, Defense(*defense));
            if template.entity_type == EntityType::Equipment {
                commands.add_component(entity, Armour {});
            }
        }
        if let Some(ranged_damage) = &template.base_ranged_damage {
            commands.add_component(entity, RangedDamage(*ranged_damage));
            if template.entity_type == EntityType::Equipment {
                commands.add_component(entity, ProjectileStack(3));
            }
        }
    }
}
