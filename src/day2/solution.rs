use std::fs;


pub fn solution(file_path: &String) -> i32
{
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path))
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut total: i32 = 0;
    for text in contents {
        let line:Vec<&str> = text.split(' ').collect();
        let player_1_move = line[0].chars().next().unwrap();
        let outcome = line[1].chars().next().unwrap();
        let value = player_1_move as i32 - 'A' as i32;

        if outcome == 'Y' {
            total += value + 1 + 3;
        }
        else if outcome == 'X'
        {
            total += ((value - 1).rem_euclid(3)) + 1;
        }
        else {
            total += ((value + 1) % 3) + 1 + 6;
        }
    }

    return total;
}