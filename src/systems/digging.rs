use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
pub fn digging(
    entity: &Entity,
    want_dig: &WantsToDig,
    #[resource] map: &mut Map,
    // #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    println!("digg0");
    let idx = map.point2d_to_index(want_dig.destination);
    map.tiles[idx] = TileType::Floor;
    commands.remove(*entity);
}
