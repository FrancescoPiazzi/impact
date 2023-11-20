use crate::common::damage::PoisonDamageType;
use crate::objects::object::Object;


// object with this trait can be applied to objects with the Poisonable trait
pub trait Applicable{}


pub struct DamagePotion{
    pub object: Object,
    pub damage: PoisonDamageType,
}

pub struct HealthPotion{
    pub object: Object,
    pub health: u32,    // TODO: refine
}

impl DamagePotion{
    pub fn new(object: Object, damage: PoisonDamageType) -> DamagePotion{
        DamagePotion{
            object: object,
            damage: damage,
        }
    }
}

impl HealthPotion{
    fn new(object: Object, health: u32) -> HealthPotion{
        HealthPotion{
            object: object,
            health: health,
        }
    }
}

impl Applicable for DamagePotion{}

impl Applicable for HealthPotion{}
