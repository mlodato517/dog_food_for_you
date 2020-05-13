use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

type Lines = Vec<Vec<String>>;
type Map = HashMap<String, Vec<String>>;

pub struct Mapping {
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

        let (dog_food_map, food_dog_map) = Self::maps_from_lines(dog_food_lines);
        let (food_ingredient_map, ingredient_food_map) =
            Self::maps_from_lines(food_ingredient_lines);
        let (ingredient_flavor_map, flavor_ingredient_map) =
            Self::maps_from_lines(ingredient_flavor_lines);

        Mapping {
            dog_food_map,
            food_ingredient_map,
            ingredient_flavor_map,
            flavor_ingredient_map,
            ingredient_food_map,
            food_dog_map,
        }
    }

    pub fn dogs(&self) -> impl Iterator<Item = &String> {
        self.dog_food_map.keys()
    }

    pub fn food_liked_by_dog<R: Rng>(&self, dog: &str, rng: &mut R) -> &str {
        let food_list = &self.dog_food_map[dog];
        &food_list[rng.gen_range(0, food_list.len())]
    }

    pub fn ingredient_in_food<R: Rng>(&self, food: &str, rng: &mut R) -> &str {
        let ingredient_list = &self.food_ingredient_map[food];
        &ingredient_list[rng.gen_range(0, ingredient_list.len())]
    }

    pub fn flavor_for_ingredient<R: Rng>(&self, ingredient: &str, rng: &mut R) -> &str {
        let flavor_list = &self.ingredient_flavor_map[ingredient];
        &flavor_list[rng.gen_range(0, flavor_list.len())]
    }

    pub fn ingredient_with_flavor<R: Rng>(&self, flavor: &str, rng: &mut R) -> &str {
        let ingredient_list = &self.flavor_ingredient_map[flavor];
        &ingredient_list[rng.gen_range(0, ingredient_list.len())]
    }

    pub fn food_with_ingredient<R: Rng>(&self, ingredient: &str, rng: &mut R) -> &str {
        let food_list = &self.ingredient_food_map[ingredient];
        &food_list[rng.gen_range(0, food_list.len())]
    }

    pub fn dog_that_likes_food<R: Rng>(&self, food: &str, rng: &mut R) -> &str {
        let dog_list = &self.food_dog_map[food];
        &dog_list[rng.gen_range(0, dog_list.len())]
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

    fn maps_from_lines(lines: Lines) -> (Map, Map) {
        let mut left_to_right_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut right_to_left_map: HashMap<String, Vec<String>> = HashMap::new();

        for ids in lines {
            left_to_right_map
                .entry(ids[0].to_owned())
                .or_insert_with(Vec::new)
                .push(ids[1].to_owned());

            right_to_left_map
                .entry(ids[1].to_owned())
                .or_insert_with(Vec::new)
                .push(ids[0].to_owned());
        }

        (left_to_right_map, right_to_left_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    /// Builds a Map (HashMap<String, Vec<String>) from something like:
    /// {
    ///   key1 => [v1, v2],
    ///   key2 => [v1]
    /// }
    macro_rules! map {
        ($($key:expr => [$($val:expr),*]),*) => {
            {
                let mut map: Map = HashMap::new();
                $(
                    let mut value: Vec<String> = Vec::new();
                    $(
                        value.push(String::from($val));
                    )*
                    map.insert(String::from($key), value);
                )*

                map
            }
        }
    }

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

        let expected_dog_food_map = map! {
            "Sparky" => ["burger","pizza"],
            "Max" => ["burger","taco"]
        };
        let expected_food_dog_map = map! {
            "burger" => ["Sparky","Max"],
            "pizza" => ["Sparky"],
            "taco" => ["Max"]
        };

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

        let expected_food_ingredient_map = map! {
            "burger" => ["cheese","tomato"],
            "pizza" => ["cheese"],
            "taco" => ["bacon"]
        };
        let expected_ingredient_food_map = map! {
            "cheese" => ["burger","pizza"],
            "tomato" => ["burger"],
            "bacon" => ["taco"]
        };

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

        let expected_ingredient_flavor_map = map! {
            "cheese" => ["salty"],
            "tomato" => ["salty","savory"],
            "bacon" => ["savory"]
        };
        let expected_flavor_ingredient_map = map! {
            "salty" => ["cheese","tomato"],
            "savory" => ["tomato","bacon"]
        };

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

        let expected_dog_food_map = map! { "Sparky" => ["burger"] };
        assert_eq!(maps.dog_food_map, expected_dog_food_map);

        let expected_food_dog_map = map! { "burger" => ["Sparky"] };
        assert_eq!(maps.food_dog_map, expected_food_dog_map);

        let expected_food_ingredient_map = map! { "burger" => ["cheese"] };
        assert_eq!(maps.food_ingredient_map, expected_food_ingredient_map);

        let expected_ingredient_food_map = map! { "cheese" => ["burger"] };
        assert_eq!(maps.ingredient_food_map, expected_ingredient_food_map);

        let expected_ingredient_flavor_map = map! { "cheese" => ["salty"] };
        assert_eq!(maps.ingredient_flavor_map, expected_ingredient_flavor_map);

        let expected_flavor_ingredient_map = map! { "salty" => ["cheese"] };
        assert_eq!(maps.flavor_ingredient_map, expected_flavor_ingredient_map);
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

        let expected_dog_food_map = map! { "Sparky" => ["burger"] };
        assert_eq!(maps.dog_food_map, expected_dog_food_map);

        let expected_food_dog_map = map! { "burger" => ["Sparky"] };
        assert_eq!(maps.food_dog_map, expected_food_dog_map);

        let expected_food_ingredient_map = map! { "burger" => ["cheese"] };
        assert_eq!(maps.food_ingredient_map, expected_food_ingredient_map);

        let expected_ingredient_food_map = map! { "cheese" => ["burger"] };
        assert_eq!(maps.ingredient_food_map, expected_ingredient_food_map);

        let expected_ingredient_flavor_map = map! { "cheese" => ["salty"] };
        assert_eq!(maps.ingredient_flavor_map, expected_ingredient_flavor_map);

        let expected_flavor_ingredient_map = map! { "salty" => ["cheese"] };
        assert_eq!(maps.flavor_ingredient_map, expected_flavor_ingredient_map);
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

        let expected_dog_food_map = map! { "Sparky" => ["burger"] };
        assert_eq!(maps.dog_food_map, expected_dog_food_map);

        let expected_food_dog_map = map! { "burger" => ["Sparky"] };
        assert_eq!(maps.food_dog_map, expected_food_dog_map);

        let expected_food_ingredient_map = map! { "burger" => ["cheese"] };
        assert_eq!(maps.food_ingredient_map, expected_food_ingredient_map);

        let expected_ingredient_food_map = map! { "cheese" => ["burger"] };
        assert_eq!(maps.ingredient_food_map, expected_ingredient_food_map);

        let expected_ingredient_flavor_map = map! { "cheese" => ["salty"] };
        assert_eq!(maps.ingredient_flavor_map, expected_ingredient_flavor_map);

        let expected_flavor_ingredient_map = map! { "salty" => ["cheese"] };
        assert_eq!(maps.flavor_ingredient_map, expected_flavor_ingredient_map);
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

        let expected_dog_food_map = map! { "Sparky" => ["burger"] };
        assert_eq!(maps.dog_food_map, expected_dog_food_map);

        let expected_food_dog_map = map! { "burger" => ["Sparky"] };
        assert_eq!(maps.food_dog_map, expected_food_dog_map);

        let expected_food_ingredient_map = map! { "burger" => ["cheese"] };
        assert_eq!(maps.food_ingredient_map, expected_food_ingredient_map);

        let expected_ingredient_food_map = map! { "cheese" => ["burger"] };
        assert_eq!(maps.ingredient_food_map, expected_ingredient_food_map);

        let expected_ingredient_flavor_map = map! { "cheese" => ["salty"] };
        assert_eq!(maps.ingredient_flavor_map, expected_ingredient_flavor_map);

        let expected_flavor_ingredient_map = map! { "salty" => ["cheese"] };
        assert_eq!(maps.flavor_ingredient_map, expected_flavor_ingredient_map);
    }
}
