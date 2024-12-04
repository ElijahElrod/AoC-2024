use regex::Regex;
use std::{
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let re = Regex::new(r"mul[(](?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})[)]").unwrap();

    if let Ok(lines) = read_lines(
        "/Users/elijahelrod/Development/projects/rust-projects/AoC-2024/day-3/src/input.txt",
    ) {
        let mut _instructions_enabled = true;
        // TODO: Add logic for flipping enable based on do() and don't()

        let mut ans = 0;
        for line in lines.map_while(Result::ok) {
            let _: Vec<u64> = re
                .captures_iter(&line)
                .map(|caps| {
                    let first_num = caps
                        .name("x")
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .expect("Parse Err");
                    let second_num = caps
                        .name("y")
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .expect("Parse Err");

                    ans += first_num * second_num;
                    ans
                })
                .collect();
        }

        println!("{}", ans)
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let input = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(input).lines())
}
