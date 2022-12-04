use std::{fs, collections::HashSet};


pub fn solution(file_path: &String) -> i32
{

    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path))
        .split("\n")
        .map(|s| s.to_string())
        .collect();

        let mut total: i32 = 0;
        for i in (0..contents.len()).step_by(3) {
            // split characters

            let set1: HashSet<char> = contents[i].chars().collect();
            let set2: HashSet<char> = contents[i + 1].chars().collect();
            let set3: HashSet<char> = contents[i + 2].chars().collect();

            let intersection = &(&set1 & &set2)& (&set3);
            intersection.iter().for_each(|s: &char| {
                if s.is_lowercase()
                {
                    total += (*s as i32 - 'a' as i32) + 1;
                }
                else 
                {
                    total += (*s as i32 - 'A' as i32) + 26 + 1;
                }
            }
            
            )
        }

    return total;
}