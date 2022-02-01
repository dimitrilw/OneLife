use serde::{Deserialize, Serialize};
use strum::EnumIter;

use super::Recordable;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AutoSettingTypes {
    AutoWorkTrue,
    AutoWorkFalse,
    AutoLivingTrue,
    AutoLivingFalse,
    AutoBuyItemTrue,
    AutoBuyItemFalse,
    AutoBuyBlessingTrue,
    AutoBuyBlessingFalse,
    AutoBuyTombTrue,
    AutoBuyTombFalse,
    AutoRebirthTrue,
    AutoRebirthFalse,
}

impl Recordable for AutoSettingTypes {
    fn to_record_key(&self) -> String {
        match self {
            AutoSettingTypes::AutoWorkTrue => "Auto Work True",
            AutoSettingTypes::AutoWorkFalse => "Auto Work False",
            AutoSettingTypes::AutoLivingTrue => "Auto Living True",
            AutoSettingTypes::AutoLivingFalse => "Auto Living False",
            AutoSettingTypes::AutoBuyItemTrue => "Auto Buy Item True",
            AutoSettingTypes::AutoBuyItemFalse => "Auto Buy Item False",
            AutoSettingTypes::AutoBuyBlessingTrue => "Auto Buy Blessing True",
            AutoSettingTypes::AutoBuyBlessingFalse => "Auto Buy Blessing False",
            AutoSettingTypes::AutoBuyTombTrue => "Auto Buy Tomb True",
            AutoSettingTypes::AutoBuyTombFalse => "Auto Buy Tomb False",
            AutoSettingTypes::AutoRebirthTrue => "Auto Rebirth True",
            AutoSettingTypes::AutoRebirthFalse => "Auto Rebirth False",
        }
        .into()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub struct Options {
    pub auto_work: bool,
    pub auto_living: bool,
    pub auto_buy_item: bool,
    pub auto_buy_blessing: bool,
    pub auto_buy_tomb: bool,
    pub auto_rebirth: bool,
    pub auto_end_early: bool,
    pub auto_end_early_criteria: f64,
    pub show_bought_items: bool,
    pub show_bought_upgrades: bool,
    pub show_recorded: bool,
    pub paused: bool,
}
