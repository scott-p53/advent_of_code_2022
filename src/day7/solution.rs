use std::{fs};
use std::collections::HashMap;

pub fn solution(file_path: &String) -> usize
{
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path))
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
        let mut total: usize = 0;
        for line in contents {
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < line.len() - 14
            {
                let window = chars[i..i+14].to_vec();
                let char_count = window.iter().fold(
                    HashMap::new(),
                    |mut acc, c|
                    {
                        *acc.entry(c).or_insert(0) += 1;
                        acc
                    }
                );

                if !char_count.iter().any(|(_, val)| val > &1)
                {
                    total = i + 14;
                    break;
                }

                i += 1;
            }
            
        }

    return total;
}