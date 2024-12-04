use std::{
    collections::HashMap,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut left = Vec::<u64>::new();
    let mut right = Vec::<u64>::new();
    let mut freq = HashMap::<u64, u64>::new();

    if let Ok(lines) =
        read_lines("/Users/elijahelrod/Development/Learning/rust-projects/aoc-2024/day-1/src/input.txt")
    {
        for line in lines.map_while(Result::ok) {
            let pair = line.split(' ').collect::<Vec<&str>>();
            left.push(
                pair.first()
                    .expect("No element at index 0")
                    .parse::<u64>()
                    .expect("Could not parse"),
            );

            let key = pair
                .last()
                .expect("No element at index 0")
                .parse::<u64>()
                .expect("Could not parse");

            right.push(key);

            if let Some(val) = freq.get(&key) {
                freq.insert(key, val + 1);
            } else {
                freq.insert(key, 1);
            }
        }
    }

    left.sort();
    right.sort();

    let mut comb_dist = 0;
    let mut sim_score = 0;
    for i in 0..left.len() {
        let key = left.get(i).expect("No element");

        if let Some(right_val) = right.get(i) {
            comb_dist += key.abs_diff(*right_val);
        }

        if let Some(val) = freq.get(&key) {
            sim_score += key * val;
        }
    }

    println!("Dist: {}", comb_dist);
    println!("Sim Score: {}", sim_score);
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let input = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(input).lines())
}
