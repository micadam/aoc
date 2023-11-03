use std::collections::HashMap;

use crate::pack::aoc23;
use crate::pack::euler;
use crate::pack::DayPack;

pub fn get_packs() -> HashMap<String, DayPack> {
    HashMap::from([
        (String::from("aoc23"), aoc23::get_pack()),
        (String::from("euler"), euler::get_pack()),
    ])
}
