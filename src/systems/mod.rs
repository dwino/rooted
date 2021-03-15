use crate::prelude::*;

mod camera_movement;
mod chasing_ai;
mod combat;
mod digging;
mod eco_entity_render;
mod eco_input;
mod eco_map_render;
mod eco_movement;
mod eco_tooltips;
mod end_input;
mod end_turn;
mod entity_render;
mod foraging_ai;
mod fov;
mod hud;
mod map_render;
mod movement;
mod random_move_ai;
mod random_patrolling_ai;
mod ranged_ai;
mod rl_input;
mod spawning_forager;
mod spawning_fruit;
mod state_switch;
mod targeting;
mod tooltips;
mod use_items;

// ROGUELIKE_MODE

pub fn build_rl_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(rl_input::rl_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(targeting::targetting_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(end_input::end_input_system())
        .build()
}

pub fn build_rl_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_rl_creature_and_plant_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_ai::random_move_ai_system())
        .add_system(ranged_ai::ranged_ai_system())
        .add_system(chasing_ai::chasing_ai_system())
        .add_system(spawning_fruit::spawning_fruit_system())
        .flush()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

// ECOSYSTEM_MODE

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(eco_input::eco_input_system())
        .flush()
        .add_system(state_switch::state_switch_system())
        .build()
}

pub fn build_logic_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(spawning_fruit::spawning_fruit_system())
        .add_system(spawning_forager::spawning_forager_system())
        .flush()
        .add_system(random_move_ai::random_move_ai_system())
        .add_system(random_patrolling_ai::random_patrolling_ai_system())
        // .add_system(spawning_fruit::spawning_fruit_system())
        // .add_system(spawning_forager::spawning_forager_system())
        .add_system(foraging_ai::foraging_ai_system())
        .flush()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(eco_movement::eco_movement_system())
        .add_system(digging::digging_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .build()
}
pub fn build_render_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(camera_movement::camera_movement_system())
        .flush()
        .add_system(eco_map_render::eco_map_render_system())
        .add_system(eco_entity_render::eco_entity_render_system())
        .add_system(eco_tooltips::eco_tooltips_system())
        .build()
}
