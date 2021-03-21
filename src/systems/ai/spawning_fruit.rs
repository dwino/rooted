use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(SpawningFruit)]
#[read_component(Fruit)]
pub fn spawning_fruit(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut fruit_spawners = <(&Point, &SpawningFruit)>::query();
    let mut rng = RandomNumberGenerator::new();
    let mut fruits_positions = <&Point>::query().filter(component::<Fruit>());
    fruit_spawners.iter(ecs).for_each(|(pos, fruit_spawn)| {
        if rng.range(0, 10) < 1 {
            let color = RGB::from_hex(fruit_spawn.template.color.clone()).expect("Bad Hex");
            let delta = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(0, -1),
                2 => Point::new(1, 0),
                _ => Point::new(0, 1),
            };

            if map.can_enter_tile(*pos + delta)
                && fruits_positions
                    .iter(ecs)
                    .filter(|fruit_position| (*pos + delta) == **fruit_position)
                    .count()
                    == 0
            {
                let fruit = commands.push((
                    *pos + delta,
                    Render {
                        color: ColorPair::new(color, RGB::from_hex("#D7E7D0").unwrap()),
                        glyph: to_cp437(fruit_spawn.template.glyph),
                    },
                    Name(fruit_spawn.template.name.clone()),
                    Item {},
                    Fruit {},
                ));
                if let Some(effects) = &fruit_spawn.template.provides {
                    effects
                        .iter()
                        .for_each(|(provides, n)| match provides.as_str() {
                            "Healing" => {
                                commands.add_component(fruit, ProvidesHealing { amount: *n })
                            }
                            "MagicMap" => commands.add_component(fruit, ProvidesDungeonMap {}),
                            _ => {
                                println!("Warning: we don't know how to provide {}", provides);
                            }
                        });
                }
            }
        }
    });
}
