use crate::prelude::*;

#[system]
#[read_component(WantEndInput)]
pub fn end_input(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let current_state = *turn_state;

    if let Some(new_state) = <&WantEndInput>::query()
        .iter(ecs)
        .find_map(|message| Some(message.0))
    {
        *turn_state = new_state;
    } else {
        println!("uhoh!");
    }
}
