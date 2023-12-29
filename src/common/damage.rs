#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DamageType {
    Physical(PhysicalDamageType),
    Elemental(ElementalDamageType),
    Poison(PoisonDamageType),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PhysicalDamageType {
    Pierce,
    Impact,
    Slash,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElementalDamageType {
    Cold,
    Heat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PoisonDamageType {
    Acid,
}

#[derive(Copy, Clone)]
pub struct Damage {
    pub damage_type: DamageType,
    pub amount: f32
}

#[derive(PartialEq, Debug)]
pub enum DamageResult{
    NoDamage,
    Damage(f32),
    Destroyed,
    Killed,
}

pub trait Damageable {
    fn damage(&mut self, damage: Damage) -> DamageResult{
        DamageResult::Damage(damage.amount)
    }
}