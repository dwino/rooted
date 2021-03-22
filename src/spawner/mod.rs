use crate::prelude::*;
mod template;
pub use template::*;

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let player = ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(
                RGB::from_hex("#D7E7D0").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            glyph: to_cp437('@'),
        },
        Health {
            current: 55,
            max: 55,
        },
        Targeting {
            targets: Vec::new(),
            current_target: None,
            index: usize::MAX,
        },
        FieldOfView::new(9),
        TargetRange::new(7),
        Damage(2),
    ));
    ecs.push((
        Name("Thorn Dart|dam:1".to_string()),
        Item {},
        Equipment {},
        Equiped(player),
        RangedDamage(2),
        ProjectileStack(12),
    ));
}

pub fn spawn_magic_droplet(ecs: &mut World, pos: Point) {
    println!("spawned droplet at{:?}", pos);
    ecs.push((
        Item,
        MagicDroplet,
        pos,
        Render {
            color: ColorPair::new(
                RGB::from_hex("#7AC5CD").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            glyph: to_cp437('♥'),
        },
        Name("Magic Droplet".to_string()),
    ));
}
