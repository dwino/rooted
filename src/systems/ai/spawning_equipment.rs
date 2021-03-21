use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(SpawningEquipment)]
#[read_component(Equipment)]
pub fn spawning_equipment(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut equipment_spawners = <(&Point, &SpawningEquipment)>::query();
    let mut rng = RandomNumberGenerator::new();
    let mut equipment_positions = <&Point>::query().filter(component::<Equipment>());
    equipment_spawners
        .iter(ecs)
        .for_each(|(pos, equipment_spawn)| {
            if rng.range(0, 10) < 1 {
                let color = RGB::from_hex(equipment_spawn.template.color.clone()).expect("Bad Hex");
                let delta = match rng.range(0, 4) {
                    0 => Point::new(-1, 0),
                    1 => Point::new(0, -1),
                    2 => Point::new(1, 0),
                    _ => Point::new(0, 1),
                };

                if map.can_enter_tile(*pos + delta)
                    && equipment_positions
                        .iter(ecs)
                        .filter(|equipment_position| (*pos + delta) == **equipment_position)
                        .count()
                        == 0
                {
                    let equipment = commands.push((
                        *pos + delta,
                        Render {
                            color: ColorPair::new(color, RGB::from_hex("#D7E7D0").unwrap()),
                            glyph: to_cp437(equipment_spawn.template.glyph),
                        },
                        Name(equipment_spawn.template.name.clone()),
                        Item {},
                        Equipment {},
                    ));
                    if let Some(damage) = &equipment_spawn.template.base_damage {
                        commands.add_component(equipment, Damage(*damage));
                        if equipment_spawn.template.entity_type == EntityType::Equipment {
                            commands.add_component(equipment, Weapon {});
                        }
                    }
                    if let Some(defense) = &equipment_spawn.template.base_defense {
                        commands.add_component(equipment, Defense(*defense));
                        if equipment_spawn.template.entity_type == EntityType::Equipment {
                            commands.add_component(equipment, Armour {});
                        }
                    }
                    if let Some(ranged_damage) = &equipment_spawn.template.base_ranged_damage {
                        commands.add_component(equipment, RangedDamage(*ranged_damage));
                        if equipment_spawn.template.entity_type == EntityType::Equipment {
                            commands.add_component(equipment, ProjectileStack(3));
                        }
                    }
                }
            }
        });
}
