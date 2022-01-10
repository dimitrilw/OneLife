use crate::engine::{character_death_update, engine_run};
use crate::game::Game;
use crate::input::activity::ActivityTypes;
use crate::input::options::Options;
use crate::input::Input;
use crate::input_recording::Inputs;
use crate::state::rebirth_stats::RebirthStats;
use crate::state::state_container::rebirth;
use crate::WORLD;

pub fn get_upgrades_up_to_current_tier(rebirth_stats: &mut RebirthStats) {
    get_upgrades_up_to_tier(rebirth_stats, rebirth_stats.tier)
}

pub fn run_until_dead(game: &mut Game) {
    while !game.state.life_stats.is_dying {
        engine_run(game);
    }
    character_death_update(game);
    assert!(game.state.life_stats.dead);
}

pub fn do_test_rebirth(game: &mut Game) {
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(game.state.rebirth_stats.clone());
    game.input = Input::new(&game.state);
}

pub fn get_upgrades_up_to_tier(rebirth_stats: &mut RebirthStats, min_tier: u32) {
    get_upgrades_up_to_tier_max_cost(rebirth_stats, min_tier, 1.0e100)
}

pub fn get_upgrades_by_max_cost(rebirth_stats: &mut RebirthStats, max_cost: f64) {
    get_upgrades_up_to_tier_max_cost(rebirth_stats, 99, max_cost)
}

pub fn get_upgrades_up_to_tier_max_cost(
    rebirth_stats: &mut RebirthStats,
    min_tier: u32,
    max_cost: f64,
) {
    for upgrade in rebirth_stats.rebirth_upgrades.iter_mut() {
        let upgrade_world = WORLD.get_rebirth_upgrade(upgrade.name);
        if upgrade_world.required_tier < min_tier && upgrade_world.purchasing_cost <= max_cost {
            upgrade.is_purchased = true;
        }
    }
}

pub fn set_full_auto(options: &mut Options) {
    options.auto_work = true;
    options.auto_living = true;
    options.auto_buy_item = true;
    options.auto_buy_tomb = true;
}

pub fn balance_activities(
    inputs: &mut Inputs,
    start_tick: u32,
    end_tick: u32,
    activities: &[ActivityTypes],
) {
    let increment = 200;
    let mut tick = start_tick;
    let mut i = 0usize;
    while tick < end_tick - increment {
        inputs.register_input_on_tick(tick, activities[i % activities.len()]);
        tick += increment;
        i += 1;
    }
}
