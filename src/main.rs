use std::env;
use std::fs;

mod argument_parser;
mod file_parser;
mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
 
    let filename = argument_parser::parse_config(&args);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let data = file_parser::parse(&contents);

    let answer = day2::part_1(&data);

    println!("{:?}", answer)

}
