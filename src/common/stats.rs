extern crate derive_more;

use derive_more::Add;

use rand::Rng;

// stats are used as part of actors descriptions but also to describe buffs and debuffs of some objects
#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct Stats{
    pub strength: StrengthStat,
    pub perception: PerceptionStat,
    pub endurance: EnduranceStat,
    pub charisma: CharismaStat,
    pub intelligence: IntelligenceStat,
    pub agility: AgilityStat,
}

#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct StrengthStat{
    pub max_carry_weight: u32,
    // base force of a blow, influenced by the weight of the weapon, strength of the actor, and wind up time
    pub base_melee_force: u32,
}

#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct PerceptionStat{
    pub vision: f32,
    pub hearing: f32,
    pub smell: f32,
    pub taste: f32,
    pub night_vision_modifier: f32,  // multiplied by vision to get cision in the dark
}

#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct EnduranceStat{
    pub stamina: u32,

    // stamina used to move for a fixed amount of time, influenced by speed, terrain, weather, equipment...
    pub base_stamina_used_for_movement: u32,

    // stamina used to fight for a fixed amount of time influenced by weapon used, combat style...
    pub base_stamina_used_for_fighting: u32,

    // health gained per unit of time
    pub health_regen: f32,
}

#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct CharismaStat{
    pub intimidation: f32,  
    pub persuasion: f32,    
    pub deception: f32,     // lying
    pub performance: f32,   // acting
}

#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct IntelligenceStat{
    pub sensorary_memory: f32,
    pub text_memory: f32,
    pub learning: f32,
    pub application: f32,
    pub creativity: f32,
}

#[derive(PartialEq, Clone, Copy, Debug, Add)]
pub struct AgilityStat{
    pub max_sprint_speed: u32,
    pub climbing: f32,
    pub riding: f32,
    pub swimming: f32,
    pub parkour: f32,
    pub stealth: f32,
    pub acrobatics: f32,
    pub pickpocting: f32,
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
    pub fn new(stamina: u32, base_stamina_used_for_movement: u32, base_stamina_used_for_fighting: u32, health_regen: f32) -> EnduranceStat{
        EnduranceStat{
            stamina: stamina,
            base_stamina_used_for_movement: base_stamina_used_for_movement,
            base_stamina_used_for_fighting: base_stamina_used_for_fighting,
            health_regen: health_regen,
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
        strength: StrengthStat,
        perception: PerceptionStat,
        endurance: EnduranceStat,
        charisma: CharismaStat,
        intelligence: IntelligenceStat,
        agility: AgilityStat,
    ) -> Stats{
        Stats{
            strength: strength,
            perception: perception,
            endurance: endurance,
            charisma: charisma,
            intelligence: intelligence,
            agility: agility,
        }
    }

    pub fn new_zero() -> Stats {
        Stats::new(
            StrengthStat::new(0, 0),
            PerceptionStat::new(0.0, 0.0, 0.0, 0.0, 0.0),
            EnduranceStat::new(0, 0, 0, 0.0),
            CharismaStat::new(0.0, 0.0, 0.0, 0.0),
            IntelligenceStat::new(0.0, 0.0, 0.0, 0.0, 0.0),
            AgilityStat::new(0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        )
    }

    pub fn new_random() -> Stats {
        let mut rng = rand::thread_rng();

        let strength = StrengthStat::new(
            rng.gen_range(1..=10),
            rng.gen_range(1..=10),
        );

        let perception = PerceptionStat::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        let endurance = EnduranceStat::new(
            rng.gen_range(1..=10),
            rng.gen_range(1..=10),
            rng.gen_range(1..=10),
            rng.gen_range(0.001..=0.005),
        );

        let charisma = CharismaStat::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        let intelligence = IntelligenceStat::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        let agility = AgilityStat::new(
            rng.gen_range(1..=10),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        Stats::new(
            strength,
            perception,
            endurance,
            charisma,
            intelligence,
            agility,
        )
    }
}