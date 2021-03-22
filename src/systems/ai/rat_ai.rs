use std::cmp::max;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(RatAi)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Equipment)]
#[read_component(Energy)]
pub fn rat_ai(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();
    let mut movers = <(Entity, &Point, &RatAi, &FieldOfView, &Energy)>::query();
    let mut player = <(Entity, &Point, &Player)>::query();
    let player_entity = player.iter(ecs).next().unwrap().0;
    let player_pos = player.iter(ecs).next().unwrap().1;
    let player_idx = map.point2d_to_index(*player_pos);
    let mut equipment = <(Entity, &Equipment, &Point)>::query();
    let mut occupied_positions = Vec::new();
    movers.iter(ecs).for_each(|(entity, pos, _, fov, energy)| {
        let mut search_targets: Vec<(usize, f32)> = Vec::new();

        let mut use_dijkstra_nav = false;
        let mut attack_player = false;
        let mut acted = false;

        let distance_to_player = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
        if distance_to_player < 1.2 {
            attack_player = true;
        };

        if attack_player {
            commands.push((
                (),
                WantsToAttack {
                    attacker: *entity,
                    victim: *player_entity,
                },
            ));
            occupied_positions.push(*pos);
            acted = true;
        }

        if !acted {
            if let Some(eq) = equipment
                .iter(ecs)
                .filter(|(_item_entity, _item, item_pos)| {
                    DistanceAlg::Pythagoras.distance2d(**item_pos, *pos) < 1.9
                })
                .find_map(|(item_entity, _item, _item_pos)| Some(item_entity))
            {
                commands.remove(*eq);
                commands.add_component(
                    *entity,
                    Energy {
                        current: energy.max,
                        max: energy.max,
                    },
                );
                occupied_positions.push(*pos);
                acted = true;
            }
        }

        if fov.visible_tiles.contains(&player_pos) {
            search_targets.push((player_idx, 0.0));
            use_dijkstra_nav = true;
        }

        if energy.current < energy.max / 2 {
            if !search_targets.is_empty() {
                search_targets = Vec::new();
            }
            search_targets.push((player_idx, 0.0));

            use_dijkstra_nav = true;

            equipment.iter(ecs).for_each(|(_entity, _equipment, pos)| {
                let idx = map.point2d_to_index(*pos);
                search_targets.push((idx, 0.0));
            });
        }

        if !acted && use_dijkstra_nav {
            let dijkstra_map =
                WeightedDijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

            if let Some(destination) = WeightedDijkstraMap::find_lowest_exit(
                &dijkstra_map,
                map.point2d_to_index(*pos),
                map,
            ) {
                let destination = map.index_to_point2d(destination);

                if !occupied_positions.contains(&destination) {
                    occupied_positions.push(destination);
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
            acted = true;
        }

        if !acted {
            commands.add_component(*entity, WantsToPatrolRandomly {})
        }

        commands.add_component(
            *entity,
            Energy {
                current: max(energy.current - 1, 0),
                max: energy.max,
            },
        );
    });
}
