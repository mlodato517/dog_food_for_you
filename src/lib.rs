mod get_line;
mod mapping;

use get_line::get_line;
use mapping::Mapping;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

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

    let mut output_file =
        BufWriter::with_capacity(4 * 1024 * 1024, File::create(output_filename).unwrap());

    let mut line = String::with_capacity(15 * 6 * walks_per_line as usize);
    for dog in mapping.dogs() {
        for _ in 0..lines_per_dog {
            let mut rng = Xoshiro256PlusPlus::from_rng(rand::thread_rng()).unwrap();
            line.clear();
            get_line(walks_per_line, dog, &mapping, &mut rng, &mut line);
            output_file.write_all(line.as_bytes()).unwrap();
        }
    }

    output_file.flush().unwrap();
}
