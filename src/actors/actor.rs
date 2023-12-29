use std::collections::HashMap;

use crate::common::damage::*;
use crate::common::position::*;
use crate::common::stats::Stats;

pub struct Actor{
    body: Body,
    base_stats: Stats,
    stat_modifiers: Stats,
    statuses: HashMap<StatusType, u32>,
}

pub struct Body{
    body_parts: Vec<BodyPart>,
    n_able_arts: u32,
    n_able_arts_available: u32,
    height: u32,
    weight: u32,
}

pub struct BodyPart{
    name: String,
    vital: bool,        // kill the actor if this is destroyed
    can_grab: bool,     // can wield a weapon
    armor_low: u32,         // anything less than this won't do damage
    armor_high: u32,        // anything more than this will destroy the body part
    x_position: XPosition,
    y_position: YPosition,
}


// TODO: move in common
pub enum StatusType{
    // control
    Stun,      // unable to act and falls to the ground
    Sleep,     // similar to stun, but can be woken up
    Taunt,     // forced to attack the taunter
    Rooted,    // unable to move lower parts of the body, but can still act
    Charm,     // forced to attack allies
    Terror,    // forced to flee
    
    // damage over time
    Burn,
    Freeze,

    // good statuses
    Invisible,
    Unstoppable,    // immune to control statuses
    Invincible,     // immune to any type of damage or control
    Immortal,       // immune to death, but can take damage or be conrtolled
    UnderwaterBreathing,

    // neutral statuses
    Ethereal,   // unable to act, but also unable to be acted upon
}


impl Actor{
    pub fn new(body: Body, base_stats: Stats) -> Actor{
        Actor{
            body: body,
            base_stats: base_stats,
            stat_modifiers: Stats::new_zero(),
            statuses: HashMap::new(),
        }
    }

    pub fn get_stats(&self) -> Stats{
        self.base_stats + self.stat_modifiers
    }
}


impl Body{
    pub fn new(body_parts: Vec<BodyPart>, height: u32, weight: u32) -> Body{
        let mut n_able_arts = 0;
        let mut n_able_arts_available = 0;
        for body_part in body_parts.iter(){
            if body_part.can_grab{
                n_able_arts += 1;
                n_able_arts_available += 1;
            }
        }
        Body{
            body_parts: body_parts,
            height: height,
            weight: weight,
            n_able_arts: n_able_arts,
            n_able_arts_available: n_able_arts_available,
        }
    }
}


impl BodyPart{
    pub fn new(
        name: String, 
        vital: bool, 
        can_grab: bool,
        armor_low: u32, 
        armor_high: u32, 
        x_position: XPosition, 
        y_position: YPosition
    ) -> BodyPart{
        if armor_low > armor_high{
            panic!("armor_low must be less than or equal to armor_high");
        }
        BodyPart{
            name: name,
            vital: vital,
            can_grab: can_grab,
            armor_low: armor_low,
            armor_high: armor_high,
            x_position: x_position,
            y_position: y_position,
        }
    }
}


impl Damageable for BodyPart {
    fn damage(&mut self, damage: Damage) -> DamageResult{
        if damage.amount < self.armor_low as f32{
            DamageResult::NoDamage
        }
        else if damage.amount > self.armor_high as f32{
            if self.vital{
                DamageResult::Killed
            }
            else{
                DamageResult::Destroyed
            }
        }
        else{
            // TODO: refine this i.e. use a third order polynomial parametrized by damage.amount 
            // that is 0 at armor_low and damage.amount at armor_high
            DamageResult::Damage(damage.amount)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_new() {
        let body = Body::new(vec![], 180, 70);
        let base_stats = Stats::new_random();
        let actor = Actor::new(body, base_stats);
        assert_eq!(actor.base_stats, base_stats);
        assert_eq!(actor.stat_modifiers, Stats::new_zero());
    }

    #[test]
    fn test_actor_get_stats() {
        let body = Body::new(vec![], 180, 70);
        let base_stats = Stats::new_random();
        let stat_modifiers = Stats::new_random();
        let mut actor = Actor::new(body, base_stats.clone());
        actor.stat_modifiers = stat_modifiers.clone();
        assert_eq!(actor.stat_modifiers, stat_modifiers);
        let stats = actor.get_stats();
        assert_eq!(stats, base_stats + stat_modifiers);
    }

    #[test]
    fn test_body_new() {
        let body_parts = vec![
            BodyPart::new("Hand".to_string(), false, true, 10, 10, XPosition::Left, YPosition::Low),
            BodyPart::new("Head".to_string(), true, false, 20, 20, XPosition::Mid, YPosition::High),
        ];
        let body = Body::new(body_parts, 180, 70);
        assert_eq!(body.height, 180);
        assert_eq!(body.weight, 70);
        assert_eq!(body.n_able_arts, 1);
    }

    #[test]
    fn test_body_part_new() {
        let body_part = BodyPart::new("Hand".to_string(), false, true, 10, 10, XPosition::Left, YPosition::Low);
        assert_eq!(body_part.name, "Hand");
        assert_eq!(body_part.vital, false);
        assert_eq!(body_part.can_grab, true);
    }

    #[test]
    fn test_body_part_damage() {
        let mut body_part = BodyPart::new("Hand".to_string(), false, true, 10, 40, XPosition::Left, YPosition::Low);
        let damage = Damage { amount: 5.0, damage_type: DamageType::Elemental(ElementalDamageType::Cold) };
        let result = body_part.damage(damage);
        assert_eq!(result, DamageResult::NoDamage);

        let damage = Damage { amount: 15.0, damage_type: DamageType::Elemental(ElementalDamageType::Cold) };
        let result = body_part.damage(damage);
        assert_eq!(result, DamageResult::Damage(15.0));

        let damage = Damage { amount: 50.0, damage_type: DamageType::Elemental(ElementalDamageType::Cold) };
        let result = body_part.damage(damage);
        assert_eq!(result, DamageResult::Destroyed);
    }

    #[test]
    fn test_body_part_kill() {
        let mut body_part = BodyPart::new("Head".to_string(), true, false, 10, 10, XPosition::Left, YPosition::Low);

        let damage = Damage { amount: 25.0, damage_type: DamageType::Physical(PhysicalDamageType::Slash) };
        let result = body_part.damage(damage);
        assert_eq!(result, DamageResult::Killed);
    }
}
