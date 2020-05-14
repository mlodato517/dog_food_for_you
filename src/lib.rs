mod get_line;
mod mapping;

use get_line::get_line;
use mapping::Mapping;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Write};

const ID_LENGTH: usize = 3;
const ID_AND_WHITESPACE_LENGTH: usize = ID_LENGTH + 1;
const STEPS_PER_WALK: usize = 6;

pub fn write_file(
    dog_food_filename: &str,
    food_ingredient_filename: &str,
    ingredient_flavor_filename: &str,
    output_filename: &str,
    lines_per_dog: u8,
    walks_per_line: u8,
) {
    let mapping = Mapping::new(
        BufReader::new(File::open(dog_food_filename).unwrap()),
        BufReader::new(File::open(food_ingredient_filename).unwrap()),
        BufReader::new(File::open(ingredient_flavor_filename).unwrap()),
    );

    let mut output_file = File::create(output_filename).unwrap();

    let line_length = ID_AND_WHITESPACE_LENGTH
        + ID_AND_WHITESPACE_LENGTH * STEPS_PER_WALK * walks_per_line as usize;

    let mut lines = vec![b' '; line_length * lines_per_dog as usize];
    for dog in mapping.dogs() {
        lines.par_chunks_mut(line_length).for_each(|mut line| {
            let mut rng = Xoshiro256PlusPlus::from_rng(rand::thread_rng()).unwrap();
            get_line(walks_per_line, dog, &mapping, &mut rng, &mut line);
        });
        output_file.write_all(&lines).unwrap();
    }
}
