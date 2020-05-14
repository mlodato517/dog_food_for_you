use super::mapping::Mapping;
use super::{ID_AND_WHITESPACE_LENGTH, ID_LENGTH, STEPS_PER_WALK};
use rand::Rng;

pub fn get_line<R: Rng>(walks_per_line: u8, dog: u16, maps: &Mapping, mut rng: R, line: &mut [u8]) {
    let line_length = ID_AND_WHITESPACE_LENGTH
        + ID_AND_WHITESPACE_LENGTH * STEPS_PER_WALK * walks_per_line as usize;
    let line = &mut line[0..line_length];

    line[0..ID_LENGTH].copy_from_slice(&maps.id(dog));

    let mut dog = dog;
    for walk_idx in 0..walks_per_line {
        let mut walk_offset = ID_AND_WHITESPACE_LENGTH
            + walk_idx as usize * STEPS_PER_WALK * ID_AND_WHITESPACE_LENGTH;

        let food = maps.food_liked_by_dog(dog, &mut rng);
        line[walk_offset..walk_offset + ID_LENGTH].copy_from_slice(&maps.id(food));
        walk_offset += ID_AND_WHITESPACE_LENGTH;

        let ingredient = maps.ingredient_in_food(food, &mut rng);
        line[walk_offset..walk_offset + ID_LENGTH].copy_from_slice(&maps.id(ingredient));
        walk_offset += ID_AND_WHITESPACE_LENGTH;

        let flavor = maps.flavor_for_ingredient(ingredient, &mut rng);
        line[walk_offset..walk_offset + ID_LENGTH].copy_from_slice(&maps.id(flavor));
        walk_offset += ID_AND_WHITESPACE_LENGTH;

        let ingredient = maps.ingredient_with_flavor(flavor, &mut rng);
        line[walk_offset..walk_offset + ID_LENGTH].copy_from_slice(&maps.id(ingredient));
        walk_offset += ID_AND_WHITESPACE_LENGTH;

        let food = maps.food_with_ingredient(ingredient, &mut rng);
        line[walk_offset..walk_offset + ID_LENGTH].copy_from_slice(&maps.id(food));
        walk_offset += ID_AND_WHITESPACE_LENGTH;

        dog = maps.dog_that_likes_food(food, &mut rng);
        line[walk_offset..walk_offset + ID_LENGTH].copy_from_slice(&maps.id(dog));
    }

    *line.last_mut().unwrap() = b'\n';
}
