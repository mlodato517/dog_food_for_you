use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufRead;
use std::str;

type Lines = Vec<Vec<String>>;
type Map = Vec<Vec<u16>>;

pub struct Mapping {
    base64_cache: Vec<[u8; 3]>,
    dog_food_map: Map,
    food_ingredient_map: Map,
    ingredient_flavor_map: Map,
    flavor_ingredient_map: Map,
    ingredient_food_map: Map,
    food_dog_map: Map,
}

impl Mapping {
    pub fn new<R>(dog_food_file: R, food_ingredient_file: R, ingredient_flavor_file: R) -> Self
    where
        R: BufRead,
    {
        let (dog_food_lines, food_ingredient_lines, ingredient_flavor_lines) =
            Self::get_filtered_lines(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        let (
            dog_food_map,
            food_dog_map,
            food_ingredient_map,
            ingredient_food_map,
            ingredient_flavor_map,
            flavor_ingredient_map,
        ) = Self::maps_from_lines(
            dog_food_lines,
            food_ingredient_lines,
            ingredient_flavor_lines,
        );

        let base64_cache = Self::build_base64_cache(
            &dog_food_map,
            &food_ingredient_map,
            &ingredient_flavor_map,
            &flavor_ingredient_map,
        );

        Mapping {
            base64_cache,
            dog_food_map,
            food_dog_map,
            food_ingredient_map,
            ingredient_food_map,
            ingredient_flavor_map,
            flavor_ingredient_map,
        }
    }

    pub fn dogs(&self) -> impl Iterator<Item = u16> {
        (0..self.dog_food_map.len()).map(|n| n as u16)
    }

    pub fn food_liked_by_dog<R: Rng>(&self, dog: u16, rng: &mut R) -> u16 {
        let food_list = &self.dog_food_map[dog as usize];
        food_list[rng.gen_range(0, food_list.len())]
    }

    pub fn ingredient_in_food<R: Rng>(&self, food: u16, rng: &mut R) -> u16 {
        let ingredient_list = &self.food_ingredient_map[food as usize];
        ingredient_list[rng.gen_range(0, ingredient_list.len())]
    }

    pub fn flavor_for_ingredient<R: Rng>(&self, ingredient: u16, rng: &mut R) -> u16 {
        let flavor_list = &self.ingredient_flavor_map[ingredient as usize];
        flavor_list[rng.gen_range(0, flavor_list.len())]
    }

    pub fn ingredient_with_flavor<R: Rng>(&self, flavor: u16, rng: &mut R) -> u16 {
        let ingredient_list = &self.flavor_ingredient_map[flavor as usize];
        ingredient_list[rng.gen_range(0, ingredient_list.len())]
    }

    pub fn food_with_ingredient<R: Rng>(&self, ingredient: u16, rng: &mut R) -> u16 {
        let food_list = &self.ingredient_food_map[ingredient as usize];
        food_list[rng.gen_range(0, food_list.len())]
    }

    pub fn dog_that_likes_food<R: Rng>(&self, food: u16, rng: &mut R) -> u16 {
        let dog_list = &self.food_dog_map[food as usize];
        dog_list[rng.gen_range(0, dog_list.len())]
    }

    pub fn id(&self, id: u16) -> [u8; 3] {
        self.base64_cache[id as usize]
    }

    fn get_lines<R: BufRead>(file: R) -> Lines {
        file.lines()
            .map(|line| line.unwrap())
            .map(|line| line.split(',').map(String::from).collect())
            .collect()
    }

    fn get_filtered_lines<R: BufRead>(
        dog_food_file: R,
        food_ingredient_file: R,
        ingredient_flavor_file: R,
    ) -> (Lines, Lines, Lines) {
        let dog_food_lines = Self::get_lines(dog_food_file);
        let foods: HashSet<&str> = dog_food_lines.iter().map(|ids| ids[1].as_str()).collect();

        let food_ingredient_lines: Vec<Vec<String>> = Self::get_lines(food_ingredient_file)
            .into_iter()
            .filter(|ids| foods.contains(ids[0].as_str()))
            .collect();
        let ingredients: HashSet<&str> = food_ingredient_lines
            .iter()
            .map(|ids| ids[1].as_str())
            .collect();

        let ingredient_flavor_lines: Vec<Vec<String>> = Self::get_lines(ingredient_flavor_file)
            .into_iter()
            .filter(|ids| ingredients.contains(ids[0].as_str()))
            .collect();
        let ingredients: HashSet<&str> = ingredient_flavor_lines
            .iter()
            .map(|ids| ids[0].as_str())
            .collect();

        let food_ingredient_lines: Vec<Vec<String>> = food_ingredient_lines
            .into_iter()
            .filter(|ids| ingredients.contains(ids[1].as_str()))
            .map(|ids| ids.into_iter().map(String::from).collect())
            .collect();
        let foods: HashSet<&str> = food_ingredient_lines
            .iter()
            .map(|ids| ids[0].as_str())
            .collect();

        let dog_food_lines: Vec<Vec<String>> = dog_food_lines
            .into_iter()
            .filter(|ids| foods.contains(ids[1].as_str()))
            .collect();

        (
            dog_food_lines,
            food_ingredient_lines,
            ingredient_flavor_lines,
        )
    }

    fn maps_from_lines(
        dog_food_lines: Lines,
        food_ingredient_lines: Lines,
        ingredient_flavor_lines: Lines,
    ) -> (Map, Map, Map, Map, Map, Map) {
        // Dogs <-> Food maps
        let mut dog_map = HashMap::new();
        let mut food_map = HashMap::new();
        for ids in &dog_food_lines {
            let new_idx = dog_map.len();
            dog_map.entry(ids[0].to_owned()).or_insert(new_idx);

            let new_idx = food_map.len();
            food_map.entry(ids[1].to_owned()).or_insert(new_idx);
        }

        let mut dog_food_map = vec![Vec::new(); dog_map.len()];
        let mut food_dog_map = vec![Vec::new(); food_map.len()];
        for ids in &dog_food_lines {
            let dog_entry = dog_map[&ids[0]];
            let food_entry = food_map[&ids[1]];
            dog_food_map[dog_entry].push(food_entry as u16);
            food_dog_map[food_entry].push(dog_entry as u16);
        }

        // Food <-> Ingredients map
        let mut ingredient_map = HashMap::new();
        for ids in &food_ingredient_lines {
            let new_idx = ingredient_map.len();
            ingredient_map.entry(ids[1].to_owned()).or_insert(new_idx);
        }

        let mut food_ingredient_map = vec![Vec::new(); food_map.len()];
        let mut ingredient_food_map = vec![Vec::new(); ingredient_map.len()];
        for ids in &food_ingredient_lines {
            let food_entry = food_map[&ids[0]];
            let ingredient_entry = ingredient_map[&ids[1]];
            food_ingredient_map[food_entry].push(ingredient_entry as u16);
            ingredient_food_map[ingredient_entry].push(food_entry as u16);
        }

        // Ingredient <-> Flavors map
        let mut flavor_map = HashMap::new();
        for ids in &ingredient_flavor_lines {
            let new_idx = flavor_map.len();
            flavor_map.entry(ids[1].to_owned()).or_insert(new_idx);
        }

        let mut ingredient_flavor_map = vec![Vec::new(); ingredient_map.len()];
        let mut flavor_ingredient_map = vec![Vec::new(); flavor_map.len()];
        for ids in &ingredient_flavor_lines {
            let ingredient_entry = ingredient_map[&ids[0]];
            let flavor_entry = flavor_map[&ids[1]];
            ingredient_flavor_map[ingredient_entry].push(flavor_entry as u16);
            flavor_ingredient_map[flavor_entry].push(ingredient_entry as u16);
        }

        // Write off original ID <-> base64 ID mappings for future consumption!
        Self::write_base64_mappings(&dog_map, &food_map, &ingredient_map, &flavor_map);

        (
            dog_food_map,
            food_dog_map,
            food_ingredient_map,
            ingredient_food_map,
            ingredient_flavor_map,
            flavor_ingredient_map,
        )
    }

    fn build_base64_cache(
        dog_food_map: &Map,
        food_ingredient_map: &Map,
        ingredient_flavor_map: &Map,
        flavor_ingredient_map: &Map,
    ) -> Vec<[u8; 3]> {
        let longest_len = [
            dog_food_map.len(),
            food_ingredient_map.len(),
            ingredient_flavor_map.len(),
            flavor_ingredient_map.len(),
        ]
        .iter()
        .copied()
        .max()
        .unwrap();

        (0..longest_len)
            .map(|n| Self::base64_encode(n as u16))
            .collect()
    }

    fn base64_encode(idx: u16) -> [u8; 3] {
        let mut buf = [0, 0, 0];
        base64::encode_config_slice(idx.to_ne_bytes(), base64::STANDARD_NO_PAD, &mut buf);
        buf
    }

    fn write_base64_mappings(
        dog_map: &HashMap<String, usize>,
        food_map: &HashMap<String, usize>,
        ingredient_map: &HashMap<String, usize>,
        flavor_map: &HashMap<String, usize>,
    ) {
        [
            ("dog_mapping.csv", dog_map),
            ("food_mapping.csv", food_map),
            ("ingredient_mapping.csv", ingredient_map),
            ("flavor_mapping.csv", flavor_map),
        ]
        .iter()
        .for_each(|(filename, map)| {
            let lines: Vec<String> = map
                .iter()
                .map(|(string_id, usize_id)| {
                    let base64_value = Self::base64_encode(*usize_id as u16);
                    let written_base64_id = str::from_utf8(&base64_value).unwrap();
                    format!("{},{}", string_id, written_base64_id)
                })
                .collect();
            fs::write(filename, lines.join("\n")).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_new_dog_food_file() {
        let food_ingredient_file =
            String::from("burger,cheese\nburger,tomato\npizza,cheese\ntaco,beef");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());
        let ingredient_flavor_file =
            String::from("cheese,salty\ntomato,salty\ntomato,savory\nbeef,savory");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let dog_food_file = String::from("Sparky,burger\nSparky,pizza\nMax,burger\nMax,taco");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());

        let expected_dog_food_map = vec![vec![0, 1], vec![0, 2]];
        let expected_food_dog_map = vec![vec![0, 1], vec![0], vec![1]];

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.dog_food_map, expected_dog_food_map);
        assert_eq!(maps.food_dog_map, expected_food_dog_map);
    }

    #[test]
    fn test_new_food_ingredient_file() {
        let dog_food_file = String::from("Sparky,burger\nSparky,pizza\nMax,burger\nMax,taco");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());
        let ingredient_flavor_file =
            String::from("cheese,salty\ntomato,salty\ntomato,savory\nbacon,savory");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let food_ingredient_file =
            String::from("burger,cheese\nburger,tomato\npizza,cheese\ntaco,bacon");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());

        let expected_food_ingredient_map = vec![vec![0, 1], vec![0], vec![2]];
        let expected_ingredient_food_map = vec![vec![0, 1], vec![0], vec![2]];

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.food_ingredient_map, expected_food_ingredient_map);
        assert_eq!(maps.ingredient_food_map, expected_ingredient_food_map);
    }

    #[test]
    fn test_new_ingredient_flavor_file() {
        let dog_food_file = String::from("Sparky,burger\nSparky,pizza\nMax,burger\nMax,taco");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());
        let food_ingredient_file =
            String::from("burger,cheese\nburger,tomato\npizza,cheese\ntaco,bacon");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());

        let ingredient_flavor_file =
            String::from("cheese,salty\ntomato,salty\ntomato,savory\nbacon,savory");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let expected_ingredient_flavor_map = vec![vec![0], vec![0, 1], vec![1]];
        let expected_flavor_ingredient_map = vec![vec![0, 1], vec![1, 2]];

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.ingredient_flavor_map, expected_ingredient_flavor_map);
        assert_eq!(maps.flavor_ingredient_map, expected_flavor_ingredient_map);
    }

    #[test]
    fn test_get_maps_filtering_food_no_ingredients() {
        let dog_food_file = String::from("Sparky,burger\nSparky,pizza");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());

        // Notice that pizza has no ingredients
        let food_ingredient_file = String::from("burger,cheese");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());

        let ingredient_flavor_file = String::from("cheese,salty");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.dog_food_map, vec![vec![0]]);
        assert_eq!(maps.food_dog_map, vec![vec![0]]);
        assert_eq!(maps.food_ingredient_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_food_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_flavor_map, vec![vec![0]]);
        assert_eq!(maps.flavor_ingredient_map, vec![vec![0]]);
    }

    #[test]
    fn test_get_maps_filtering_ingredients_no_flavors() {
        let dog_food_file = String::from("Sparky,burger");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());

        let food_ingredient_file = String::from("burger,cheese\nburger,tomato");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());

        // Notice that the tomato has no flavors
        let ingredient_flavor_file = String::from("cheese,salty");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.dog_food_map, vec![vec![0]]);
        assert_eq!(maps.food_dog_map, vec![vec![0]]);
        assert_eq!(maps.food_ingredient_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_food_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_flavor_map, vec![vec![0]]);
        assert_eq!(maps.flavor_ingredient_map, vec![vec![0]]);
    }

    #[test]
    fn test_get_maps_filtering_ingredient_no_food() {
        let dog_food_file = String::from("Sparky,burger");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());

        // Notice that no foods use tomato
        let food_ingredient_file = String::from("burger,cheese");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());

        let ingredient_flavor_file = String::from("cheese,salty\ntomato,salty");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.dog_food_map, vec![vec![0]]);
        assert_eq!(maps.food_dog_map, vec![vec![0]]);
        assert_eq!(maps.food_ingredient_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_food_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_flavor_map, vec![vec![0]]);
        assert_eq!(maps.flavor_ingredient_map, vec![vec![0]]);
    }

    #[test]
    fn test_get_maps_filtering_food_no_dog() {
        // Notice no dog ate pizza
        let dog_food_file = String::from("Sparky,burger");
        let dog_food_file = BufReader::new(dog_food_file.as_bytes());

        let food_ingredient_file = String::from("burger,cheese\npizza,tomato");
        let food_ingredient_file = BufReader::new(food_ingredient_file.as_bytes());

        let ingredient_flavor_file = String::from("cheese,salty\ntomato,salty");
        let ingredient_flavor_file = BufReader::new(ingredient_flavor_file.as_bytes());

        let maps = Mapping::new(dog_food_file, food_ingredient_file, ingredient_flavor_file);

        assert_eq!(maps.dog_food_map, vec![vec![0]]);
        assert_eq!(maps.food_dog_map, vec![vec![0]]);
        assert_eq!(maps.food_ingredient_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_food_map, vec![vec![0]]);
        assert_eq!(maps.ingredient_flavor_map, vec![vec![0]]);
        assert_eq!(maps.flavor_ingredient_map, vec![vec![0]]);
    }
}
