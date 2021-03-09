use crate::prelude::*;
mod template;
pub use template::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
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
            current: 100,
            max: 100,
        },
        FieldOfView::new(10),
        Damage(5),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(
                RGB::from_hex("#5D76CB").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            glyph: to_cp437('☼'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
