use crate::prelude::*;

#[system]
#[read_component(WantsEndInput)]
pub fn end_input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some((message_entity, new_state)) = <(Entity, &WantsEndInput)>::query()
        .iter(ecs)
        .find_map(|(message_entity, message)| Some((message_entity, message.0)))
    {
        *turn_state = new_state;
        commands.remove(*message_entity);
    }
}
