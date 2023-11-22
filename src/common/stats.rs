// stats are used as part of actors descriptions but also to describe buffs and debuffs of some objects

#[derive(Clone, Copy)]
pub struct Stats{
    raw_strength: u32,
    strength: StrengthStat,
    raw_perception: u32,
    perception: PerceptionStat,
    raw_endurance: u32,
    endurance: EnduranceStat,
    raw_charisma: u32,
    charisma: CharismaStat,
    raw_intelligence: u32,
    intelligence: IntelligenceStat,
    raw_agility: u32,
    agility: AgilityStat,
}

#[derive(Clone, Copy)]
pub struct StrengthStat{
    max_carry_weight: u32,
    // base force of a blow, influenced by the weight of the weapon, strength of the actor, and wind up time
    base_melee_force: u32,
}

#[derive(Clone, Copy)]
pub struct PerceptionStat{
    vision: f32,
    hearing: f32,
    smell: f32,
    taste: f32,
    night_vision_modifier: f32,  // multiplied by vision to get cision in the dark
}

#[derive(Clone, Copy)]
pub struct EnduranceStat{
    stamina: u32,

    // stamina used to move for a fixed amount of time, influenced by speed, terrain, weather, equipment...
    base_stamina_used_for_movement: u32,

    // stamina used to fight for a fixed amount of time influenced by weapon used, combat style...
    base_stamina_used_for_fighting: u32
}

#[derive(Clone, Copy)]
pub struct CharismaStat{
    intimidation: f32,  
    persuasion: f32,    
    deception: f32,     // lying
    performance: f32,   // acting
}

#[derive(Clone, Copy)]
pub struct IntelligenceStat{
    sensorary_memory: f32,
    text_memory: f32,
    learning: f32,
    application: f32,
    creativity: f32,
}

#[derive(Clone, Copy)]
pub struct AgilityStat{
    max_sprint_speed: u32,
    climbing: f32,
    riding: f32,
    swimming: f32,
    parkour: f32,
    stealth: f32,
    acrobatics: f32,
    pickpocting: f32,
}

impl StrengthStat{
    pub fn new(max_carry_weight: u32, base_melee_force: u32) -> StrengthStat{
        StrengthStat{
            max_carry_weight: max_carry_weight,
            base_melee_force: base_melee_force,
        }
    }
}

impl PerceptionStat{
    pub fn new(vision: f32, hearing: f32, smell: f32, taste: f32, night_vision_modifier: f32) -> PerceptionStat{
        PerceptionStat{
            vision: vision,
            hearing: hearing,
            smell: smell,
            taste: taste,
            night_vision_modifier: night_vision_modifier,
        }
    }
}

impl EnduranceStat{
    pub fn new(stamina: u32, base_stamina_used_for_movement: u32, base_stamina_used_for_fighting: u32) -> EnduranceStat{
        EnduranceStat{
            stamina: stamina,
            base_stamina_used_for_movement: base_stamina_used_for_movement,
            base_stamina_used_for_fighting: base_stamina_used_for_fighting,
        }
    }
}

impl CharismaStat{
    pub fn new(intimidation: f32, persuasion: f32, deception: f32, performance: f32) -> CharismaStat{
        CharismaStat{
            intimidation: intimidation,
            persuasion: persuasion,
            deception: deception,
            performance: performance,
        }
    }
}

impl IntelligenceStat{
    pub fn new(sensorary_memory: f32, text_memory: f32, learning: f32, application: f32, creativity: f32) -> IntelligenceStat{
        IntelligenceStat{
            sensorary_memory: sensorary_memory,
            text_memory: text_memory,
            learning: learning,
            application: application,
            creativity: creativity,
        }
    }
}

impl AgilityStat{
    pub fn new(max_sprint_speed: u32, climbing: f32, riding: f32, swimming: f32, parkour: f32, stealth: f32, acrobatics: f32, pickpocting: f32) -> AgilityStat{
        AgilityStat{
            max_sprint_speed: max_sprint_speed,
            climbing: climbing,
            riding: riding,
            swimming: swimming,
            parkour: parkour,
            stealth: stealth,
            acrobatics: acrobatics,
            pickpocting: pickpocting,
        }
    }
}

impl Stats{
    pub fn new(
        raw_strength: u32,
        strength: StrengthStat,
        raw_perception: u32,
        perception: PerceptionStat,
        raw_endurance: u32,
        endurance: EnduranceStat,
        raw_charisma: u32,
        charisma: CharismaStat,
        raw_intelligence: u32,
        intelligence: IntelligenceStat,
        raw_agility: u32,
        agility: AgilityStat,
    ) -> Stats{
        Stats{
            raw_strength: raw_strength,
            strength: strength,
            raw_perception: raw_perception,
            perception: perception,
            raw_endurance: raw_endurance,
            endurance: endurance,
            raw_charisma: raw_charisma,
            charisma: charisma,
            raw_intelligence: raw_intelligence,
            intelligence: intelligence,
            raw_agility: raw_agility,
            agility: agility,
        }
    }
}