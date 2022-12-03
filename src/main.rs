use std::{env};
use advent_of_code::day2::{solution::solution};

fn main() {
    //let args: Vec<String> = env::args().collect();
    let file_path = &String::from("../inputs/day_2.txt");//&args[1];
    print!("Solution={}.\n", solution(file_path));
}
