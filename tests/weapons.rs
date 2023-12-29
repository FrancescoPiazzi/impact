use impact::objects::weapons::common::{Weapon, Defend};
use impact::common::damage::{Damage, DamageType, PhysicalDamageType};
use impact::objects::object::Object;


#[test]
fn test_weapon_new() {
    let object = Object::new(1, String::from("Test Weapon"), String::from("This is a test weapon"), 10, 20);
    let weapon = Weapon::new(object.clone(), 10, 0.5, PhysicalDamageType::Impact);

    assert_eq!(weapon.object, object);
    assert_eq!(weapon.attack_speed, 10);
    assert_eq!(weapon.defense_multiplier, 0.5);
    assert_eq!(weapon.damage_type, PhysicalDamageType::Impact);
}


#[test]
fn test_weapon_defend() {
    let object = Object::new(1, String::from("Test Weapon"), String::from("This is a test weapon"), 10, 20);
    let weapon = Weapon::new(object, 10, 0.5, PhysicalDamageType::Impact);

    let damage = Damage {
        damage_type: DamageType::Physical(PhysicalDamageType::Impact),
        amount: 100.0,
    };
    let result = weapon.defend(damage.clone());

    assert_eq!(result.damage_type, damage.damage_type);
    assert_eq!(result.amount, 50.0);  // 100.0 * 0.5
}
