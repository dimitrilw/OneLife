use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Items {
    pub money: f64,
}

impl Items {
    pub fn new() -> Items {
        Items { money: 0.0 }
    }
}
