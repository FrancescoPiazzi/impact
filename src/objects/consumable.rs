use crate::common::damage::PoisonDamageType;
use crate::common::status::StatusType;
use crate::common::stats::Stats;
use crate::objects::object::Object;


// object with this trait can be applied to objects with the Poisonable trait
pub trait Applicable{}


// object that can be consumed
// a consumed object can give different statuses and stats for differents amounts of time
pub struct Consumable {
    pub object: Object,
    pub effects: (Vec<(StatusType, u32)>, Vec<(Stats, u32)>),
}


impl Consumable {
    pub fn new(
        object: Object,
        effects: (Vec<(StatusType, u32)>, Vec<(Stats, u32)>),
    ) -> Consumable {
        Consumable {
            object: object,
            effects: effects,
        }
    }
}

impl Applicable for Consumable{}
