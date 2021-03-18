use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Foraging)]
#[read_component(Energy)]
#[read_component(Health)]
#[read_component(SpawningForager)]
#[read_component(Entity)]

pub fn foraging(#[resource] map: &mut Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &Foraging, &Energy)>::query();
    let mut nests = <(Entity, &Point, &Energy)>::query().filter(component::<SpawningForager>());
    let (nest_entity, nest_energy) = nests
        .iter(ecs)
        .find_map(|(nest_entity, nest_pos, nest_energy)| Some((nest_entity, nest_energy)))
        .unwrap();

    movers
        .iter(ecs)
        .for_each(|(forager_entity, forager_pos, _foraging, forager_energy)| {
            let mut nest_weight: f32 = 0.0;
            let mut forage_weight: f32 = 0.0;

            if forager_energy.current < forager_energy.max {
                forage_weight = -1000.0;
                nest_weight = 1000.0
            } else {
                forage_weight = 1000.0;
                nest_weight = -1000.0
            }

            let mut targets = Vec::new();

            targets.push((map.forage_map.nest_positions[0], nest_weight));

            &map.forage_map.forage_positions.iter().for_each(|u| {
                targets.push((*u, forage_weight));
            });

            let mut dijkstra_map =
                WeightedDijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &targets[..], map, 1024.0);

            let idx = map.point2d_to_index(*forager_pos);
            if let Some(destination_idx) =
                WeightedDijkstraMap::find_lowest_exit(&dijkstra_map, idx, map)
            {
                let destination_point = map.index_to_point2d(destination_idx);

                if forager_energy.current > 0
                    && DistanceAlg::Pythagoras.distance2d(
                        map.index_to_point2d(map.forage_map.nest_positions[0]),
                        destination_point,
                    ) <= 1.0
                {
                    println!("near nest");
                    commands.add_component(
                        *nest_entity,
                        Energy {
                            current: forager_energy.current + nest_energy.current,
                            max: nest_energy.max,
                        },
                    );
                    commands.add_component(
                        *forager_entity,
                        Energy {
                            current: 0,
                            max: forager_energy.max,
                        },
                    );
                    // commands.flush(ecs);
                } else if DistanceAlg::Pythagoras.distance2d(
                    map.index_to_point2d(map.forage_map.forage_positions[0]),
                    destination_point,
                ) <= 1.0
                {
                    println!("near food");
                    commands.add_component(
                        *forager_entity,
                        Energy {
                            current: forager_energy.current + 1,
                            max: forager_energy.max,
                        },
                    );
                } else if map.can_enter_tile(destination_point) {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *forager_entity,
                            destination: destination_point,
                        },
                    ));
                }
            }
        });
}
