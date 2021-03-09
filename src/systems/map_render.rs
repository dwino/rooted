use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
    ecs: &SubWorld,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = fov.iter(ecs).next().unwrap();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt)
                    | map.revealed_tiles[map.point2d_to_index(pt)])
            {
                let idx = map.point2d_to_index(pt);
                let colorpair = if player_fov.visible_tiles.contains(&pt) {
                    theme.in_fov_colorpair_to_render(map.tiles[idx])
                } else {
                    theme.out_fov_colorpair_to_render(map.tiles[idx])
                };

                let glyph = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(colorpair.fg, colorpair.bg),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
