use std::{fs};

pub fn solution(file_path: &String) -> i32
{
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path))
        .split("\n")
        .map(|s| s.to_string())
        .collect();

        let mut total: i32 = 0;
        for line in contents {
            let pairs: Vec<&str> = line.split(",").collect();
            let (range1, range2): (Vec<&str>, Vec<&str>) = (pairs[0].split("-").collect(), pairs[1].split("-").collect());

             let (range_1_lower, range_1_upper): (i32, i32) = (range1[0].parse().expect("Unable to parse lower range1"), range1[1].parse().expect("Unable to prase upper range1"));
             let (range_2_lower, range_2_upper): (i32, i32) = (range2[0].parse().expect("Unable to prase lower range2"), range2[1].parse().expect("Unable to parse upper range2"));   
            
            if !(range_1_upper < range_2_lower ||
                range_2_upper < range_1_lower)
            {
                total += 1;
            }
        }

    return total;
}