#[derive(Copy, Clone)]
pub enum DamageType {
    Physical(PhysicalDamageType),
    Elemental(ElementalDamageType),
    Poison(PoisonDamageType),
}

#[derive(Copy, Clone)]
pub enum PhysicalDamageType {
    Pierce,
    Impact,
    Slash,
}

#[derive(Copy, Clone)]
pub enum ElementalDamageType {
    Cold,
    Heat,
}

#[derive(Copy, Clone)]
pub enum PoisonDamageType {
    Acid,
}

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
    fn damage(&mut self, damage: Damage) -> DamageResult;
}