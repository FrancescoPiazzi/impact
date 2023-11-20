use crate::common::damage::{Damage, DamageType, PhysicalDamageType};
use crate::objects::object::Object;
use crate::objects::consumable::*;

type Apllied = Option<Box<dyn Applicable>>;

trait Defend{
    fn defend(&self, damage: Damage) -> Damage;
}

trait MeleeAttack{
    fn normal_attack(&self) -> Damage;
    fn charged_attack(&self) -> Damage;
}

trait RangedAttack{
    fn shoot(&self, projectile: Projectile) -> Damage;
}

trait Poisonable{
    fn apply_potion(&mut self, applicable: Box<dyn Applicable>);
}

struct Weapon{
    pub object: Object,
    pub attack_speed: u32,
    pub defense_multiplier: f32,
    pub damage_type: PhysicalDamageType,
}

struct MeleeWeapon{
    pub weapon: Weapon,
    pub applicable: Apllied,
    pub base_attack_damage: u32,
    pub normal_attack_speed: u32,
    pub charged_attack_damage_multiplier: f32,
    pub charged_attack_speed_multiplier: u32,
}

struct RangedWeapon{
    pub weapon: Weapon,
    pub base_attack_damage_multiplier: f32,
    pub normal_attack_speed: u32,
    pub melee_attack_damage: u32,
    pub melee_attack_speed: u32,
}

struct Projectile{
    pub object: Object,
    pub applicable: Apllied,
    pub base_damage: u32,
}

impl Weapon{
    fn new(
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

impl MeleeWeapon{
    fn new(
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

impl RangedWeapon{
    fn new(
        weapon: Weapon, 
        base_attack_damage_multiplier: f32, 
        normal_attack_speed: u32, 
        melee_attack_damage: u32, 
        melee_attack_speed: u32
    ) -> RangedWeapon{
        RangedWeapon{
            weapon: weapon,
            base_attack_damage_multiplier: base_attack_damage_multiplier,
            normal_attack_speed: normal_attack_speed,
            melee_attack_damage: melee_attack_damage,
            melee_attack_speed: melee_attack_speed,
        }
    }
}

impl Projectile{
    fn new(object: Object, base_damage: u32) -> Projectile{
        Projectile{
            object: object,
            applicable: Option::None,
            base_damage: base_damage,
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

impl RangedAttack for RangedWeapon{
    fn shoot(&self, projectile: Projectile) -> Damage{
        let damage = Damage{
            damage_type: DamageType::Physical(self.weapon.damage_type),
            amount: projectile.base_damage as f32 * self.base_attack_damage_multiplier,
        };
        damage
    }
}

impl Poisonable for MeleeWeapon{
    fn apply_potion(&mut self, applicable: Box<dyn Applicable>){
        self.applicable = Option::Some(applicable);
    }
}


#[cfg(test)]
mod tests {
    use crate::common::damage::PoisonDamageType;

    use super::*;

    #[test]
    fn test_weapon_defend() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Weapon"), String::from("This is a test weapon"), 10, 20), // replace with actual object
            10,
            0.5,
            PhysicalDamageType::Impact, // replace with actual damage type
        );
        let damage = Damage {
            damage_type: DamageType::Physical(PhysicalDamageType::Impact), // replace with actual damage type
            amount: 100.0,
        };
        let result = weapon.defend(damage);
        assert_eq!(result.amount, 50.0);
    }

    #[test]
    fn test_melee_weapon_normal_attack() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Melee Weapon"), String::from("This is a test melee weapon"), 10, 20), // replace with actual object
            10,
            0.5,
            PhysicalDamageType::Impact, // replace with actual damage type
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
            Object::new(1, String::from("Test Melee Weapon"), String::from("This is a test melee weapon"), 10, 20), // replace with actual object
            10,
            0.5,
            PhysicalDamageType::Slash, // replace with actual damage type
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
    fn test_ranged_weapon_shoot() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Ranged Weapon"), String::from("This is a test ranged weapon"), 10, 20), // replace with actual object
            10,
            0.5,
            PhysicalDamageType::Slash, // replace with actual damage type
        );
        let ranged_weapon = RangedWeapon::new(
            weapon,
            2.0,
            10,
            100,
            20,
        );
        let projectile = Projectile::new(
            Object::new(1, String::from("Test Ranged Weapon"), String::from("This is a test ranged weapon"), 10, 20), // replace with actual object
            50,
        );
        let result = ranged_weapon.shoot(projectile);
        assert_eq!(result.amount, 100.0);
    }

    #[test]
    fn test_melee_weapon_apply_potion() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Ranged Weapon"), String::from("This is a test ranged weapon"), 10, 20), // replace with actual object
            10,
            0.5,
            PhysicalDamageType::Slash, // replace with actual damage type
        );
        let mut melee_weapon = MeleeWeapon::new(
            weapon,
            100,
            10,
            2.0,
            20,
        );
        let object = Object::new(1, String::from("Test Poison"), String::from("This is a test poison"), 10, 20);
        let potion = Box::new(DamagePotion::new(object, PoisonDamageType::Acid)); // replace with actual potion
        melee_weapon.apply_potion(potion);
        assert!(melee_weapon.applicable.is_some());
    }
}