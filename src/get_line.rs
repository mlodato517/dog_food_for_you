use super::mapping::Mapping;
use rand::Rng;

pub fn get_line<R: Rng>(
    walks_per_line: u8,
    dog: &str,
    maps: &Mapping,
    mut rng: R,
    line: &mut String,
) {
    line.push_str(dog);

    let mut dog = dog;
    for _ in 0..walks_per_line {
        let food = maps.food_liked_by_dog(dog, &mut rng);
        line.push(' ');
        line.push_str(&food);

        let ingredient = maps.ingredient_in_food(food, &mut rng);
        line.push(' ');
        line.push_str(&ingredient);

        let flavor = maps.flavor_for_ingredient(ingredient, &mut rng);
        line.push(' ');
        line.push_str(&flavor);

        let ingredient = maps.ingredient_with_flavor(flavor, &mut rng);
        line.push(' ');
        line.push_str(&ingredient);

        let food = maps.food_with_ingredient(ingredient, &mut rng);
        line.push(' ');
        line.push_str(&food);

        dog = maps.dog_that_likes_food(food, &mut rng);
        line.push(' ');
        line.push_str(&dog);
    }

    line.push('\n')
}
