use super::mapping::Mapping;
use rand::Rng;

pub fn get_line<R: Rng>(walks_per_line: u8, dog: u16, maps: &Mapping, mut rng: R) -> String {
    let mut line = Vec::with_capacity(walks_per_line as usize * 6 + 1);
    line.push(maps.dog_id(dog));

    let mut dog = dog;
    for _ in 0..walks_per_line {
        let food = maps.food_liked_by_dog(dog, &mut rng);
        line.push(maps.food_id(food));

        let ingredient = maps.ingredient_in_food(food, &mut rng);
        line.push(maps.ingredient_id(ingredient));

        let flavor = maps.flavor_for_ingredient(ingredient, &mut rng);
        line.push(maps.flavor_id(flavor));

        let ingredient = maps.ingredient_with_flavor(flavor, &mut rng);
        line.push(maps.ingredient_id(ingredient));

        let food = maps.food_with_ingredient(ingredient, &mut rng);
        line.push(maps.food_id(food));

        dog = maps.dog_that_likes_food(food, &mut rng);
        line.push(maps.dog_id(dog));
    }

    line.join(" ") + "\n"
}
