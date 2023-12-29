use crate::common::damage::*;
use crate::objects::weapons::common::*;
use crate::objects::consumable::Applicable;


pub trait MeleeAttack{
    fn normal_attack(&self) -> Damage;
    fn charged_attack(&self) -> Damage;
}


pub struct MeleeWeapon{
    pub weapon: Weapon,
    pub applicable: Applied,
    pub base_attack_damage: u32,
    pub normal_attack_speed: u32,
    pub charged_attack_damage_multiplier: f32,
    pub charged_attack_speed_multiplier: u32,
}


impl MeleeWeapon{
    pub fn new(
        weapon: Weapon, 
        base_attack_damage: u32, 
        normal_attack_speed: u32, 
        charged_attack_damage_multiplier: f32, 
        charged_attack_speed_multiplier: u32
    ) -> MeleeWeapon{
        MeleeWeapon{
            weapon: weapon,
            applicable: Option::None,
            base_attack_damage: base_attack_damage,
            normal_attack_speed: normal_attack_speed,
            charged_attack_damage_multiplier: charged_attack_damage_multiplier,
            charged_attack_speed_multiplier: charged_attack_speed_multiplier,
        }
    }
}


impl MeleeAttack for MeleeWeapon{
    fn normal_attack(&self) -> Damage {
        let damage = Damage{
            damage_type: DamageType::Physical(self.weapon.damage_type),
            amount: self.base_attack_damage as f32,
        };
        damage
    }

    fn charged_attack(&self) -> Damage {
        let damage = Damage{
            damage_type: DamageType::Physical(self.weapon.damage_type),
            amount: self.base_attack_damage as f32 * self.charged_attack_damage_multiplier,
        };
        damage
    }
}


impl Poisonable for MeleeWeapon{
    fn apply_potion_on_object(&mut self, applicable: Box<dyn Applicable>){
        self.applicable = Option::Some(applicable);
    }
}


#[cfg(test)]
mod tests {
    use crate::objects::object::Object;
    use crate::objects::consumable::Consumable;
    use crate::objects::weapons::common::*;
    use crate::common::status::StatusType::*;
    
    use super::*;

    #[test]
    fn test_melee_weapon_normal_attack() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Melee Weapon"), String::from("This is a test melee weapon"), 10, 20),
            10,
            0.5,
            PhysicalDamageType::Impact,
        );
        let melee_weapon = MeleeWeapon::new(
            weapon,
            100,
            10,
            2.0,
            20,
        );
        let result = melee_weapon.normal_attack();
        assert_eq!(result.amount, 100.0);
    }

    #[test]
    fn test_melee_weapon_charged_attack() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Melee Weapon"), String::from("This is a test melee weapon"), 10, 20),
            10,
            0.5,
            PhysicalDamageType::Slash,
        );
        let melee_weapon = MeleeWeapon::new(
            weapon,
            100,
            10,
            2.0,
            20,
        );
        let result = melee_weapon.charged_attack();
        assert_eq!(result.amount, 200.0);
    }


    #[test]
    fn test_melee_weapon_apply_potion() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Ranged Weapon"), String::from("This is a test ranged weapon"), 10, 20),
            10,
            0.5,
            PhysicalDamageType::Slash,
        );
        let mut melee_weapon = MeleeWeapon::new(
            weapon,
            100,
            10,
            2.0,
            20,
        );
        let object = Object::new(1, String::from("Test potion"), String::from("This is a test potion"), 10, 20);
        let potion = Box::new(Consumable::new(object, (vec![(Terror, 10)], vec![])));
        melee_weapon.apply_potion_on_object(potion);
        assert!(melee_weapon.applicable.is_some());
    }
}