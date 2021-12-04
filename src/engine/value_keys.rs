use crate::input::stat::StatTypes;
use crate::input::work::WorkTypes;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
// use std::mem::variant_count;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyValues {
    //Other
    Happiness,
    Money,
    Coins,
    Health,
    //Stats
    Str,
    Int,
    Cha,
    Con,
    Dex,
    Faith,
    //Work
    Mines,
    Latrine,
    GalleyRower,
    Fields,
    Mill,
    Weaver,
    Fisherman,
    Farmer,
    //Soldiers
    BagageBoy,
    Slinger,
    Peltasts,
    Pikeman,
    FootCompanion,
    Hypaspists,
    LightCavalery,
    //
}

impl From<StatTypes> for KeyValues {
    fn from(stat: StatTypes) -> Self {
        match stat {
            StatTypes::Str => KeyValues::Str,
            StatTypes::Cha => KeyValues::Cha,
            StatTypes::Con => KeyValues::Con,
            StatTypes::Int => KeyValues::Int,
            StatTypes::Dex => KeyValues::Dex,
            StatTypes::Faith => KeyValues::Faith,
        }
    }
}

impl From<WorkTypes> for KeyValues {
    fn from(work: WorkTypes) -> Self {
        match work {
            WorkTypes::Mines => KeyValues::Mines,
            WorkTypes::Latrine => KeyValues::Latrine,
            WorkTypes::GalleyRower => KeyValues::GalleyRower,
            WorkTypes::Fields => KeyValues::Fields,
            WorkTypes::Mill => KeyValues::Mill,
            WorkTypes::Weaver => KeyValues::Weaver,
            WorkTypes::Fisherman => KeyValues::Fisherman,
            WorkTypes::Farmer => KeyValues::Farmer,
            WorkTypes::BagageBoy => KeyValues::BagageBoy,
            WorkTypes::Slinger => KeyValues::Slinger,
            WorkTypes::Peltasts => KeyValues::Peltasts,
            WorkTypes::Pikeman => KeyValues::Pikeman,
            WorkTypes::FootCompanion => KeyValues::FootCompanion,
            WorkTypes::Hypaspists => KeyValues::Hypaspists,
            WorkTypes::LightCavalery => KeyValues::LightCavalery,
        }
    }
}

// pub const HOUSING_SIZE: usize = variant_count::<KeyValues>();
