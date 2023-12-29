use std::collections::HashMap;

use crate::objects::object::Object;
use crate::common::damage::DamageType;

trait Clothing{
    fn protect_element(&self);
}

trait Armor{
    fn protect_attack(&self);
}

pub struct ArmorPiece{
    pub object: Object,
    pub armor_low: HashMap<DamageType, u32>,
    pub armor_high: u32,
}