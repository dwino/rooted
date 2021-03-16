use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Foraging)]
#[read_component(FieldOfView)]
#[read_component(Health)]
pub fn foraging(#[resource] map: &mut Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &Foraging, &FieldOfView)>::query();

    map.forage_map.update_tiles(map.tiles.clone());

    let forage_targets = &map.forage_map.forage_positions;
    let mut dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &forage_targets,
        &map.forage_map,
        1024.0,
    );

    let mut rng = RandomNumberGenerator::new();
    if rng.range(0, 10) < 5 {
        println!("test");
        build_dwino(&mut dijkstra_map, forage_targets, map);
    }
    println!("1:{:?}", dijkstra_map.map[1]);

    movers.iter(ecs).for_each(|(entity, pos, _foraging, _fov)| {
        let idx = map.point2d_to_index(*pos);
        if let Some(destination) =
            DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &map.forage_map)
        {
            let destination = map.index_to_point2d(destination);

            if map.can_enter_tile(destination) {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            } else {
                commands.push((
                    (),
                    WantsToDig {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
