use crate::objects::object::Object;

enum Biome{
    Mountain,
    Plains,
    Forest,
    HotDesert,
    ColdDesert,
    Tundra,
    Jungle,
    Ocean,
    Swamp,
    Savanna,
    Taiga,
    Glacier,
    Island,
    LavaOcean,
    Sky,
}

enum World{
    Earth,
    Mars,
    Venus,
    Mercury,
    Moon,
    Europa,
    Titan,
    Hell,
    Heaven,
    Purgatory,
}

enum TileType{
    Grass,
    Dirt,
    Sand,
    Ice,
    Snow,
    Stone,
    Floor,
    Leaves,
}


// all objects that can be found naturally in the world
enum WorldObjectType{
    Tree,
    Rock,
    Bush,
    Flower,
}

enum BuildingType{
    House,
    Castle,
    Tower,
    Wall,
}

struct WorldObject{
    object: Object,
    object_type: WorldObjectType,
}