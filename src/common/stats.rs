use rand::Rng;
use std::ops::Add;

// stats are used as part of actors descriptions but also to describe buffs and debuffs of some objects

#[derive(PartialEq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct StrengthStat{
    max_carry_weight: u32,
    // base force of a blow, influenced by the weight of the weapon, strength of the actor, and wind up time
    base_melee_force: u32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PerceptionStat{
    vision: f32,
    hearing: f32,
    smell: f32,
    taste: f32,
    night_vision_modifier: f32,  // multiplied by vision to get cision in the dark
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct EnduranceStat{
    stamina: u32,

    // stamina used to move for a fixed amount of time, influenced by speed, terrain, weather, equipment...
    base_stamina_used_for_movement: u32,

    // stamina used to fight for a fixed amount of time influenced by weapon used, combat style...
    base_stamina_used_for_fighting: u32
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CharismaStat{
    intimidation: f32,  
    persuasion: f32,    
    deception: f32,     // lying
    performance: f32,   // acting
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct IntelligenceStat{
    sensorary_memory: f32,
    text_memory: f32,
    learning: f32,
    application: f32,
    creativity: f32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
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

    pub fn new_zero() -> Stats {
        Stats::new(
            0,
            StrengthStat::new(0, 0),
            0,
            PerceptionStat::new(0.0, 0.0, 0.0, 0.0, 0.0),
            0,
            EnduranceStat::new(0, 0, 0),
            0,
            CharismaStat::new(0.0, 0.0, 0.0, 0.0),
            0,
            IntelligenceStat::new(0.0, 0.0, 0.0, 0.0, 0.0),
            0,
            AgilityStat::new(0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        )
    }

    pub fn new_random() -> Stats {
        let mut rng = rand::thread_rng();

        let raw_strength = rng.gen_range(1..=10);
        let strength = StrengthStat::new(
            rng.gen_range(1..=10),
            rng.gen_range(1..=10),
        );

        let raw_perception = rng.gen_range(1..=10);
        let perception = PerceptionStat::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        let raw_endurance = rng.gen_range(1..=10);
        let endurance = EnduranceStat::new(
            rng.gen_range(1..=10),
            rng.gen_range(1..=10),
            rng.gen_range(1..=10),
        );

        let raw_charisma = rng.gen_range(1..=10);
        let charisma = CharismaStat::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        let raw_intelligence = rng.gen_range(1..=10);
        let intelligence = IntelligenceStat::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        );

        let raw_agility = rng.gen_range(1..=10);
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
            raw_strength,
            strength,
            raw_perception,
            perception,
            raw_endurance,
            endurance,
            raw_charisma,
            charisma,
            raw_intelligence,
            intelligence,
            raw_agility,
            agility,
        )
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            raw_strength: self.raw_strength + other.raw_strength,
            strength: self.strength + other.strength,
            raw_perception: self.raw_perception + other.raw_perception,
            perception: self.perception + other.perception,
            raw_endurance: self.raw_endurance + other.raw_endurance,
            endurance: self.endurance + other.endurance,
            raw_charisma: self.raw_charisma + other.raw_charisma,
            charisma: self.charisma + other.charisma,
            raw_intelligence: self.raw_intelligence + other.raw_intelligence,
            intelligence: self.intelligence + other.intelligence,
            raw_agility: self.raw_agility + other.raw_agility,
            agility: self.agility + other.agility,
        }
    }
}

impl Add for StrengthStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            max_carry_weight: self.max_carry_weight + other.max_carry_weight,
            base_melee_force: self.base_melee_force + other.base_melee_force,
        }
    }
}

impl Add for PerceptionStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            vision: self.vision + other.vision,
            hearing: self.hearing + other.hearing,
            smell: self.smell + other.smell,
            taste: self.taste + other.taste,
            night_vision_modifier: self.night_vision_modifier + other.night_vision_modifier,
        }
    }
}

impl Add for EnduranceStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            stamina: self.stamina + other.stamina,
            base_stamina_used_for_movement: self.base_stamina_used_for_movement + other.base_stamina_used_for_movement,
            base_stamina_used_for_fighting: self.base_stamina_used_for_fighting + other.base_stamina_used_for_fighting,
        }
    }
}

impl Add for CharismaStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            intimidation: self.intimidation + other.intimidation,
            persuasion: self.persuasion + other.persuasion,
            deception: self.deception + other.deception,
            performance: self.performance + other.performance,
        }
    }
}

impl Add for IntelligenceStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            sensorary_memory: self.sensorary_memory + other.sensorary_memory,
            text_memory: self.text_memory + other.text_memory,
            learning: self.learning + other.learning,
            application: self.application + other.application,
            creativity: self.creativity + other.creativity,
        }
    }
}

impl Add for AgilityStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            max_sprint_speed: self.max_sprint_speed + other.max_sprint_speed,
            climbing: self.climbing + other.climbing,
            riding: self.riding + other.riding,
            swimming: self.swimming + other.swimming,
            parkour: self.parkour + other.parkour,
            stealth: self.stealth + other.stealth,
            acrobatics: self.acrobatics + other.acrobatics,
            pickpocting: self.pickpocting + other.pickpocting,
        }
    }
}