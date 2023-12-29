use crate::common::damage::{Damage, PhysicalDamageType};
use crate::objects::object::Object;
use crate::objects::consumable::*;


pub type Applied = Option<Box<dyn Applicable>>;

pub trait Defend{
    fn defend(&self, damage: Damage) -> Damage;
}


pub trait Poisonable{
    fn apply_potion_on_object(&mut self, applicable: Box<dyn Applicable>);
}


pub struct Weapon{
    pub object: Object,
    pub attack_speed: u32,
    pub defense_multiplier: f32,
    pub damage_type: PhysicalDamageType,
}


impl Weapon{
    pub fn new(
        object: Object, 
        attack_speed: u32, 
        defense_multiplier: f32, 
        damage_type: PhysicalDamageType,
    ) -> Weapon{
        Weapon{
            object: object,
            attack_speed: attack_speed,
            defense_multiplier: defense_multiplier,
            damage_type: damage_type,
        }
    }
}


impl Defend for Weapon{
    fn defend(&self, damage: Damage) -> Damage{
        let mut new_damage = damage;
        new_damage.amount *= self.defense_multiplier;
        new_damage
    }

}


#[cfg(test)]
mod tests {
    use crate::common::damage::DamageType;
    use crate::objects::weapons::common::*;


    #[test]
    fn test_weapon_defend() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Weapon"), String::from("This is a test weapon"), 10, 20),
            10,
            0.5,
            PhysicalDamageType::Impact,
        );
        let damage = Damage {
            damage_type: DamageType::Physical(PhysicalDamageType::Impact),
            amount: 100.0,
        };
        let result = weapon.defend(damage);
        assert_eq!(result.amount, 50.0);
    }
}