use std::{fs};

pub fn solution(file_path: &String) -> String
{
    let binding = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path));

    let contents: Vec<&str>  = binding.split("\n").collect();
    
    let split_index = contents.iter().position(|s| s.is_empty()).expect("Unable to find line between cargo and moves");

    let sizes: Vec<&str> = contents[split_index-1].split_whitespace().collect();
    let size: usize = sizes.last().expect("Sizes vector should have 1 element").parse().expect("Unable to parse last size");

    let mut cargo: Vec<Vec<char>> = Vec::with_capacity(size);
    (0..size).for_each(| _ | cargo.push(Vec::new()));

    for i in (0..split_index-1).rev()
    {
        let line: Vec<char> = contents[i].chars().collect();
        let mut index = 0;
        let mut j = 0;
        while j < line.len() - 1
        {
            if line[j] == '['
            {
                // items are format [X]
                cargo[index].push(line[j + 1]);
                index += 1;
                j += 3;
            }
            else if line[j] == ' ' && line[j + 1] == ' ' 
            {
                index += 1;
                j += 4;
            }
            else 
            {
                j += 1;
            }   
        }
    }

    for i in split_index+1..contents.len()
    {
        let operations: Vec<&str> = contents[i].split_ascii_whitespace().collect();
        let moves = operations[1].parse::<usize>().expect("Unable to parse move value");
        let from_index =  operations[3].parse::<usize>().expect("Unable to parse from index");
        let to_index =  operations[5].parse::<usize>().expect("Unable to parse to index");

        let multi_move: Vec<char> = Vec::new();
        for _ in 1..moves+1
        {
            let item = cargo[from_index-1].pop().expect("Unable to pop values");
            multi_move.push(item);
        }

        multi_move.reverse();
        multi_move.iter().for_each(|item| cargo[to_index-1].push(*item));

    }
    
    let mut result = "".to_string();
    for v in cargo
    {
        result.push(*v.last().expect("Unable to get result"));
    }

    return result;
}