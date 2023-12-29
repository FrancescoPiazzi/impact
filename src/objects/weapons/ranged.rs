use crate::objects::object::Object;
use crate::objects::weapons::common::*;
use crate::common::damage::*;


pub struct Projectile{
    pub object: Object,
    pub applicable: Applied,
    pub base_damage: u32,
}


pub trait RangedAttack{
    fn shoot(&self, projectile: Projectile) -> Damage;
}


pub struct RangedWeapon{
    pub weapon: Weapon,
    pub base_attack_damage_multiplier: f32,
    pub normal_attack_speed: u32,
    pub melee_attack_damage: u32,
    pub melee_attack_speed: u32,
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


impl RangedWeapon{
    pub fn new(
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
    pub fn new(object: Object, base_damage: u32) -> Projectile{
        Projectile{
            object: object,
            applicable: Option::None,
            base_damage: base_damage,
        }
    }
}



#[cfg(test)]
mod tests{
    use crate::objects::object::Object;
    use crate::objects::weapons::common::*;

    use super::*;

    #[test]
    fn test_ranged_weapon_shoot() {
        let weapon = Weapon::new(
            Object::new(1, String::from("Test Ranged Weapon"), String::from("This is a test ranged weapon"), 10, 20),
            10,
            0.5,
            PhysicalDamageType::Slash,
        );
        let ranged_weapon = RangedWeapon::new(
            weapon,
            2.0,
            10,
            100,
            20,
        );
        let projectile = Projectile::new(
            Object::new(1, String::from("Test Ranged Weapon"), String::from("This is a test ranged weapon"), 10, 20),
            50,
        );
        let result = ranged_weapon.shoot(projectile);
        assert_eq!(result.amount, 100.0);
    }

}