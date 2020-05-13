extern crate structopt;

use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "dog_food_for_you",
    about = "Writes a file for dog food recommendations"
)]
struct Opt {
    #[structopt(
        short = "n",
        long = "lines-per-dog",
        help = "Can be 1 to 255.",
        default_value = "128"
    )]
    lines_per_dog: u8,

    #[structopt(
        short = "w",
        long = "walks-per-line",
        help = "Can be 1 to 255.",
        default_value = "64"
    )]
    walks_per_line: u8,

    #[structopt(long = "dog-food-file", default_value = "dog_food_lines.csv")]
    dog_food_filename: String,

    #[structopt(
        long = "food-ingredients-file",
        default_value = "food_ingredient_lines.csv"
    )]
    food_ingredients_filename: String,

    #[structopt(
        long = "ingredients-flavor-file",
        default_value = "ingredient_flavor_lines.csv"
    )]
    ingredients_flavor_filename: String,

    #[structopt(short = "o", long = "output-file", default_value = "output.txt")]
    output_filename: String,
}

fn main() {
    let start = Instant::now();
    let opt = Opt::from_args();

    dog_food_for_you::write_file(
        &opt.dog_food_filename,
        &opt.food_ingredients_filename,
        &opt.ingredients_flavor_filename,
        &opt.output_filename,
        opt.lines_per_dog,
        opt.walks_per_line,
    );

    println!("Done! Took {}ms", start.elapsed().as_millis());
}
