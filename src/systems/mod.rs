use crate::prelude::*;

mod ai;
mod combat;
mod fov;
mod rl;
mod targeting;
mod use_items;

// ROGUELIKE_MODE

pub fn build_rl_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(rl::input::input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(targeting::targetting_system())
        .flush()
        .add_system(rl::map_render::map_render_system())
        .add_system(rl::entity_render::entity_render_system())
        .add_system(rl::hud::hud_system())
        .add_system(rl::tooltips::tooltips_system())
        .add_system(rl::end_input::end_input_system())
        .build()
}

pub fn build_rl_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(rl::movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(rl::map_render::map_render_system())
        .add_system(rl::entity_render::entity_render_system())
        .add_system(rl::hud::hud_system())
        .add_system(rl::end_turn::end_turn_system())
        .build()
}

pub fn build_rl_creature_and_plant_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(ai::random_move::random_move_system())
        .add_system(ai::rat_ai::rat_ai_system())
        .add_system(ai::ant_ai::ant_ai_system())
        .flush()
        .add_system(ai::random_patrolling::random_patrolling_system())
        .flush()
        .add_system(ai::spawning_fruit::spawning_fruit_system())
        .add_system(ai::spawning_equipment::spawning_equipment_system())
        .flush()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(rl::movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(rl::map_render::map_render_system())
        .add_system(rl::entity_render::entity_render_system())
        .add_system(rl::hud::hud_system())
        .add_system(rl::end_turn::end_turn_system())
        .build()
}
