use crate::prelude::*;

#[system]
#[read_component(WantsToMoveCamera)]
pub fn camera_movement(
    #[resource] map: &mut Map,
    #[resource] camera: &mut EcoCamera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    println!("camera");

    if let Some((message_entity, delta)) = <(Entity, &WantsToMoveCamera)>::query()
        .iter(ecs)
        .find_map(|(message_entity, message)| Some((message_entity, message.delta)))
    {
        camera.center_on_camera_move(delta);

        commands.remove(*message_entity);
    }
}
