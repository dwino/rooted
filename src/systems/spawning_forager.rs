use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(SpawningForager)]
#[read_component(Fruit)]
pub fn spawning_forager(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut forager_spawners = <(&Point, &SpawningForager)>::query();
    let mut rng = RandomNumberGenerator::new();
    forager_spawners.iter(ecs).for_each(|(pos, spawner)| {
        println!("spawn!");
        if rng.range(0, 10) < 1 {
            let ant = commands.push((
                *pos,
                Render {
                    color: ColorPair::new(
                        RGB::from_hex("#E3CF57").unwrap(),
                        RGB::from_hex("#D7E7D0").unwrap(),
                    ),
                    glyph: to_cp437('a'),
                },
                Name("Ant".to_string()),
                Creature {},
                FieldOfView::new(7),
                Health { current: 2, max: 2 },
                Targetable {},
                Foraging {},
            ));
        }
    });
}
