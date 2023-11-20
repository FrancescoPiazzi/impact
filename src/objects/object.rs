pub struct Object{
    pub id: u32,
    pub name: String,
    pub description: String,
    pub weight: u32,
    pub volume: u32,
}

impl Object{
    pub fn new(id: u32, name: String, description: String, weight: u32, volume: u32) -> Object{
        Object{
            id: id,
            name: name,
            description: description,
            weight: weight,
            volume: volume,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_creation() {
        let id = 1;
        let name = String::from("Test Object");
        let description = String::from("This is a test object");
        let weight = 10;
        let volume = 20;

        let test_object = Object::new(id, name.clone(), description.clone(), weight, volume);

        assert_eq!(test_object.id, id);
        assert_eq!(test_object.name, name);
        assert_eq!(test_object.description, description);
        assert_eq!(test_object.weight, weight);
        assert_eq!(test_object.volume, volume);
    }
}