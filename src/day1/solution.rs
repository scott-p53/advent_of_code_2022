use std::fs;
use std::collections::BinaryHeap;

pub fn solution(file_path: &String) -> i32
{    
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path))
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut current_count = 0;
    let mut heap = BinaryHeap::new();
    for text in contents {
        if text.is_empty()
        {
            heap.push(current_count);
            current_count = 0;
            continue;
        }

        current_count += text.parse::<i32>().unwrap();
    }

    let total = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap(); 
    return total;
}
