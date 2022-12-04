use std::{env};
use advent_of_code::day3::{solution::solution};

fn main() {
    //let args: Vec<String> = env::args().collect();
    let file_path = &String::from("../inputs/day_3.txt");//&args[1];
    print!("Solution={}.\n", solution(file_path));
}
