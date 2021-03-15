use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[read_component(Equipment)]
#[read_component(Equiped)]
#[read_component(Weapon)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    println!("useitem0");
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            println!("useitem1");

            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }

                if let Ok(_equipment) = item.get_component::<Equipment>() {
                    commands.remove_component::<Carried>(activate.item);
                    commands.add_component(activate.item, Equiped(activate.used_by));

                    if let Ok(e) = ecs.entry_ref(activate.item) {
                        if e.get_component::<Weapon>().is_ok() {
                            println!("useitem2");

                            <(Entity, &Equiped, &Weapon)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == activate.used_by)
                                .for_each(|(e, _, _)| {
                                    commands.remove(*e);
                                })
                        }
                        if e.get_component::<ProjectileStack>().is_ok() {
                            <(Entity, &Equiped, &ProjectileStack)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == activate.used_by)
                                .for_each(|(e, _, _)| {
                                    commands.remove(*e);
                                })
                        }
                    }
                } else {
                    commands.remove(activate.item);
                }
            }
            commands.remove(*entity);
            println!("useitem3");
        });

    for heal in &healing_to_apply {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
    println!("useitemend");
}
