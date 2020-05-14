mod get_line;
mod mapping;

extern crate crossbeam_channel;

use crossbeam_channel::{unbounded, Receiver, Sender};
use get_line::get_line;
use mapping::Mapping;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::thread;

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

    let (sender, receiver): (Sender<String>, Receiver<String>) = unbounded();

    thread::spawn(move || {
        for line in receiver {
            output_file.write_all(line.as_bytes()).unwrap();
        }
        output_file.flush().unwrap();
    });

    for dog in mapping.dogs() {
        (0..lines_per_dog).into_par_iter().for_each(|_| {
            let mut rng = rand::thread_rng();
            let line = get_line(walks_per_line, &dog, &mapping, &mut rng);
            sender.clone().send(line).unwrap();
        });
    }
}
