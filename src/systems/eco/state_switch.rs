use crate::prelude::*;

#[system]
#[read_component(WantsStateSwitch)]
pub fn state_switch(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut EcoState,
) {
    if let Some((message_entity, new_state)) = <(Entity, &WantsStateSwitch)>::query()
        .iter(ecs)
        .find_map(|(message_entity, message)| Some((message_entity, message.0)))
    {
        *turn_state = new_state;
        commands.remove(*message_entity);
    }
}
