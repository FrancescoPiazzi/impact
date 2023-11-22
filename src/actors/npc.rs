use crate::actors::actor::Actor;


enum Role{
    Bandit,
    Guard,
    Shopkeeper,
    Farmer,
    Blacksmith,
    Noble,
    Mage,
}

struct NPC{
    actor: Actor,
    role: Role,
}