use std::i32::MAX;

use crate::{prelude::*, turn_state};

#[system]
#[read_component(Point)]
#[read_component(Player)]
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
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Creature>());
    let (player_entity, player_pos) = players
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos)))
        .unwrap();
    let mut player_fov = <&FieldOfView>::query().filter(component::<Player>());

    let mut delta = Point::new(i32::MAX, i32::MAX);
    let consumed_turn_delta = Point::new(i32::MAX - 1, i32::MAX - 1);

    if let Some(key) = *key {
        match key {
            VirtualKeyCode::H => {
                delta = Point::new(-1, 0);
            }
            VirtualKeyCode::L => {
                delta = Point::new(1, 0);
            }
            VirtualKeyCode::K => {
                delta = Point::new(0, -1);
            }
            VirtualKeyCode::J => {
                delta = Point::new(0, 1);
            }
            VirtualKeyCode::Y => {
                delta = Point::new(-1, -1);
            }
            VirtualKeyCode::U => {
                delta = Point::new(1, -1);
            }
            VirtualKeyCode::B => {
                delta = Point::new(-1, 1);
            }
            VirtualKeyCode::N => {
                delta = Point::new(1, 1);
            }
            VirtualKeyCode::Space => {
                delta = Point::new(0, 0);
            }
            VirtualKeyCode::G => {
                let mut items = <(Entity, &Item, &Point)>::query();
                items
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _item, _item_pos)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player_entity));
                    });
            }
            VirtualKeyCode::F => {
                // IF TARGETAVAILABLE
                // if let Some(closest) = enemies
                //     .iter(ecs)
                //     .filter(|(_, pos)| player_fov.visible_tiles.contains(&pos))
                //     .find_map(|(entity, _)| Some(*entity))
                if let Some(target) = <&Targeting>::query()
                    .iter(ecs)
                    .find_map(|targeting| targeting.current_target)
                {
                    commands.push((
                        (),
                        WantsToRangedAttack {
                            attacker: player_entity,
                            victim: target,
                        },
                    ));
                    *turn_state = TurnState::PlayerTurn;
                    println!("shootattempt");
                } else {
                    println!("nothing targeted");
                }
            }
            VirtualKeyCode::Tab => {
                if let Some((_, projectile)) = <(&Equiped, &ProjectileStack)>::query()
                    .iter(ecs)
                    .filter(|(equiped, _)| equiped.0 == player_entity)
                    .next()
                {
                    if projectile.0 >= 1 {
                        commands.push(((), WantsCycleTarget {}));
                        *turn_state = TurnState::AwaitingInput;
                        println!("dit lukt nog");
                    }
                }
            }
            VirtualKeyCode::Key1 => {
                use_item(0, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key2 => {
                use_item(1, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key3 => {
                use_item(2, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key4 => {
                use_item(3, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key5 => {
                use_item(4, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key6 => {
                use_item(5, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key7 => {
                use_item(6, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key8 => {
                use_item(7, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            VirtualKeyCode::Key9 => {
                use_item(8, ecs, commands, player_entity);
                delta = consumed_turn_delta;
            }
            _ => (),
        };

        if (delta.x != 0 || delta.y != 0) && (delta.x < i32::MAX - 1 && delta.y < i32::MAX - 1) {
            let destination = player_pos + delta;
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
            *turn_state = TurnState::PlayerTurn;
        }

        //TODO: check if consumed turn in a less hacky way
        if delta.x == i32::MAX || delta.y == i32::MAX {
            *turn_state = TurnState::AwaitingInput;
        } else if delta.x == i32::MAX - 1 || delta.y == i32::MAX - 1 {
            *turn_state = TurnState::PlayerTurn;
        }
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer, player_entity: Entity) {
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
}
