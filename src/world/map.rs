use crate::world::world_elements::*;

struct LocalMap{
    tiles: Vec<Vec<Tile>>,
    width: u32,
    height: u32,
}

struct Location{
    name: String,
    description: String,
    x: u32,
    y: u32,
}

struct Tile{
    tyle_type: TileType,
    objects: Set<WorldObject>,
}

struct Map{
    biomes: Vec<Vec<Biome>>,
    locations: HashMap<Location, LocalMap>,
    width: u32,
    height: u32,
}

impl Map{
    pub fn new(width: u32, height: u32) -> Map{
        Map{
            biomes: Vec::new(),
            locations: HashMap::new(),
            width: width,
            height: height,
        }
    }
}