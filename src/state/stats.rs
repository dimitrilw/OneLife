use crate::input::stat::{StatTypes, STAT_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

pub struct BaseStats {
    pub str: f64,
    pub int: f64,
    pub cha: f64,
    pub con: f64,
    pub dex: f64,
    pub faith: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Stat {
    pub name: StatTypes,
    pub value: f64,
    pub next_level_progress: f64,
    pub next_level_percentage: f64,
}

impl Stat {
    pub fn new(stat: StatTypes, init_value: f64) -> Stat {
        Stat {
            name: stat,
            value: init_value,
            next_level_progress: 0.0,
            next_level_percentage: 0.0,
        }
    }
}

pub fn get_stats(initial_values: [f64; STAT_SIZE]) -> [Stat; STAT_SIZE] {
    let mut stats: [MaybeUninit<Stat>; STAT_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in StatTypes::iter() {
        stats[name as usize].write(Stat::new(name, initial_values[name as usize]));
    }
    unsafe { mem::transmute(stats) }
}

pub fn get_stats_base(base: BaseStats) -> [Stat; STAT_SIZE] {
    let initial_values = [base.str, base.int, base.cha, base.con, base.dex, base.faith];
    get_stats(initial_values)
}
