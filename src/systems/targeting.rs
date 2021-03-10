use crate::prelude::*;

#[system]
#[read_component(WantsCycleTarget)]
#[read_component(Point)]
#[read_component(FieldOfView)]
#[write_component(Targeting)]
#[read_component(Player)]
pub fn targetting(#[resource] map: &Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    //1. Check WantsCycleTarget message
    if let Some(message) = <(Entity, &WantsCycleTarget)>::query()
        .iter_mut(ecs)
        .map(|(message_entity, _)| message_entity)
        .next()
    {
        //2. Query PlayerEntity, Fov & Targetting
        let (player_entity, player_pos, player_fov, player_targetting) =
            <(Entity, &Point, &FieldOfView, &Targeting)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .find_map(|(e, p, f, t)| Some((e, p, f, t)))
                .unwrap();

        //3. Check targets in Fov
        let mut possible_targets = <(Entity, &Targetable, &Point)>::query();
        let mut targets = possible_targets
            .iter(ecs)
            .filter(|(_, _, pt)| player_fov.visible_tiles.contains(&pt))
            .map(|(e, _, pt)| (*e, DistanceAlg::Pythagoras.distance2d(*player_pos, *pt)))
            .collect::<Vec<(Entity, f32)>>();

        targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut new_index = 0;
        let current_target = if targets.is_empty() {
            None
        } else {
            let mut target = None;
            if player_targetting.index == targets.len() - 1 {
                new_index = usize::MAX;
                target = None
            } else {
                if player_targetting.index == usize::MAX {
                    new_index = 0;
                } else {
                    new_index = player_targetting.index + 1;
                }
                target = Some(targets[new_index].0);
            }
            target
        };
        commands.add_component(
            *player_entity,
            Targeting {
                targets,
                current_target,
                index: new_index,
            },
        );
    }

    //3. Check targets in Fov
}
