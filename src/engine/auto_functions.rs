use crate::{
    game::Game,
    input::options::AutoSettingTypes,
    world_content::{
        boost_item::translate_boost_item, housing::translate_housing, tomb::translate_tomb,
    },
    WORLD,
};

pub fn auto_work(game: &mut Game) {
    let current_work = &WORLD.works[game.input.work as usize];
    for work in game.state.works.iter() {
        let work_world = &WORLD.works[work.name as usize];
        let same_type = current_work.work_type == work_world.work_type;
        if work.name > current_work.name && same_type && work.is_unlocked && work.is_visible {
            game.input.work = work.name;
        }
    }
}

pub fn auto_living(game: &mut Game) {
    let current_housing = translate_housing(game.input.housing);
    for housing in game.state.housing.iter() {
        let housing_world = translate_housing(housing.name);
        let can_afford = housing_world.upkeep < game.state.items.income;
        let better_housing = housing.name > current_housing.name;
        if better_housing && housing.is_unlocked && can_afford {
            game.input.housing = housing.name;
        }
    }
}

pub fn auto_buy_item(game: &mut Game) {
    for item in game.state.items.boost_items.iter_mut() {
        let world_item = translate_boost_item(item.name);
        let can_afford = game.state.items.money >= world_item.purchasing_cost;
        if !item.is_purchased && can_afford && item.is_visible {
            item.is_purchased = true;
            game.state.items.money -= world_item.purchasing_cost;
        }
    }
}

pub fn auto_buy_tomb(game: &mut Game) {
    for tomb in game.state.tombs.iter_mut() {
        let world_tomb = translate_tomb(tomb.name);
        let can_afford = game.state.items.money >= world_tomb.purchasing_cost;
        if !tomb.is_purchased && can_afford && tomb.is_visible {
            tomb.is_purchased = true;
            game.state.items.money -= world_tomb.purchasing_cost;
        }
    }
}

pub fn register_auto_settings(game: &mut Game) {
    if game.meta_data.options.auto_work {
        game.register_input(AutoSettingTypes::AutoWorkTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoWorkFalse)
    };
    if game.meta_data.options.auto_living {
        game.register_input(AutoSettingTypes::AutoLivingTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoLivingFalse)
    };
    if game.meta_data.options.auto_buy_item {
        game.register_input(AutoSettingTypes::AutoBuyItemTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyItemFalse)
    };
    if game.meta_data.options.auto_buy_tomb {
        game.register_input(AutoSettingTypes::AutoBuyTombTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyTombFalse)
    };
}
