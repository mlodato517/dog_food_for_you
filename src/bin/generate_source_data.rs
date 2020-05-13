extern crate rand;
extern crate structopt;

use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "generate_source_data", about = "Generates random data.")]
struct Opt {
    #[structopt(long = "num-dogs", help = "1-65535.", default_value = "5000")]
    num_dogs: u16,

    #[structopt(long = "num-foods", help = "1-65535.", default_value = "5000")]
    num_foods: u16,

    #[structopt(long = "num-ingredients", help = "1-65535.", default_value = "500")]
    num_ingredients: u16,

    #[structopt(long = "num-flavors", help = "1-255.", default_value = "10")]
    num_flavors: u8,

    #[structopt(long = "num-dog-food-lines", help = "0-255.", default_value = "100")]
    num_dog_food_lines: u8,

    #[structopt(
        long = "num-food-ingredient-lines",
        help = "0-255.",
        default_value = "50"
    )]
    num_food_ingredient_lines: u8,

    #[structopt(
        long = "num-ingredient-flavor-lines",
        help = "0-255.",
        default_value = "10"
    )]
    num_ingredient_flavor_lines: u8,
}
fn main() {
    let opt = Opt::from_args();

    if opt.num_dogs < 1 {
        panic!("num-dogs must be at least 1")
    }
    if opt.num_foods < 1 {
        panic!("num-foods must be at least 1")
    }
    if opt.num_ingredients < 1 {
        panic!("num-ingredients must be at least 1")
    }
    if opt.num_flavors < 1 {
        panic!("num-flavors must be at least 1")
    }

    let rng = thread_rng();
    let dogs: Vec<String> = (0..opt.num_dogs)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect::<String>())
        .map(|id| format!("dog-{}", id))
        .collect();
    let foods: Vec<String> = (0..opt.num_foods)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect::<String>())
        .map(|id| format!("food-{}", id))
        .collect();
    let ingredients: Vec<String> = (0..opt.num_ingredients)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect::<String>())
        .map(|id| format!("ingredient-{}", id))
        .collect();
    let flavors: Vec<String> = (0..opt.num_flavors)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect::<String>())
        .map(|id| format!("flavor-{}", id))
        .collect();

    write_association(
        "dog_food_lines.csv",
        &dogs,
        &foods,
        opt.num_dog_food_lines as usize,
    );
    write_association(
        "food_ingredient_lines.csv",
        &foods,
        &ingredients,
        opt.num_food_ingredient_lines as usize,
    );
    write_association(
        "ingredient_flavor_lines.csv",
        &ingredients,
        &flavors,
        opt.num_ingredient_flavor_lines as usize,
    );
}

fn write_association(
    filename: &str,
    items: &[String],
    associated_items: &[String],
    num_lines: usize,
) {
    let mut rng = thread_rng();
    let mut writer = BufWriter::new(File::create(filename).unwrap());
    for item in items {
        for _ in 0..num_lines {
            let line = format!("{},{}\n", item, associated_items.choose(&mut rng).unwrap());
            writer.write_all(line.as_bytes()).unwrap();
        }
    }
    writer.flush().unwrap();
}
