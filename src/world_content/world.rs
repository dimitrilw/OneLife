use super::boost_item::{get_boost_items, BoostItem};
use super::housing::{get_housing, Housing};
use super::tier::{init_tiers, Tier};
use super::work::{get_works, Work};
use crate::input::housing::HOUSING_SIZE;
use crate::input::work::WORK_SIZE;
use serde::Serialize;

#[derive(Serialize)]
pub struct World {
    pub tiers: Vec<Tier>,
    pub works: [Work; WORK_SIZE],
    pub housing: [Housing; HOUSING_SIZE],
    pub boost_items: Vec<BoostItem>,
}

impl World {
    pub fn new() -> World {
        World {
            tiers: init_tiers(),
            works: get_works(),
            housing: get_housing(),
            boost_items: get_boost_items(),
        }
    }
}
