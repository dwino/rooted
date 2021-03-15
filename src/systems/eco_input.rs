use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Creature)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Equiped)]
#[read_component(ProjectileStack)]
#[read_component(Weapon)]
#[read_component(FieldOfView)]
#[read_component(Targeting)]
#[read_component(WantsCycleTarget)]
pub fn eco_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut EcoState,
) {
    if let Some(key) = *key {
        match key {
            VirtualKeyCode::Left => move_camera(ecs, commands, Point::new(-10, 0)),
            VirtualKeyCode::Right => move_camera(ecs, commands, Point::new(10, 0)),
            VirtualKeyCode::Up => move_camera(ecs, commands, Point::new(0, -10)),
            VirtualKeyCode::Down => move_camera(ecs, commands, Point::new(0, 10)),
            VirtualKeyCode::Space => match turn_state {
                EcoState::Play => pause(commands),
                EcoState::Pause => play(commands),
                _ => {}
            },
            _ => send_end_input_message(commands, EcoState::Play),
        };
    }
}
fn move_camera(ecs: &mut SubWorld, commands: &mut CommandBuffer, delta: Point) {
    commands.push(((), WantsToMoveCamera { delta }));
}

fn pause(commands: &mut CommandBuffer) {
    commands.push(((), WantsStateSwitch(EcoState::Pause)));
}
fn play(commands: &mut CommandBuffer) {
    commands.push(((), WantsStateSwitch(EcoState::Play)));
}
fn send_end_input_message(commands: &mut CommandBuffer, new_state: EcoState) {
    commands.push(((), WantsStateSwitch(new_state)));
}
