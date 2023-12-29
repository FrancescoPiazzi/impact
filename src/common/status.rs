#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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