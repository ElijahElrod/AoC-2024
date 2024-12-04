use std::{
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines(
        "/Users/elijahelrod/Development/Learning/rust-projects/aoc-2024/day-2/src/input.txt",
    ) {
        let mut ans = 0;
        for line in lines.map_while(Result::ok) {
            let levels = line
                .split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<u64>, _>>()
                .unwrap();

            if is_monotonic_change(levels) {
                ans += 1;
            }
        }

        println!("Safe reports {}", ans);
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let input = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(input).lines())
}

fn is_monotonic_change(levels: Vec<u64>) -> bool {
    let first_elem = levels.first().expect("Empty");
    let last_elem = levels.last().expect("Empty");
    let should_be_increasing = first_elem.lt(last_elem);
    let mut prev_elem = first_elem;
    let mut curr_elem: &u64;
    
    for i in 1..levels.len() {
        curr_elem = levels.get(i).expect("No elem found at index");
        match should_be_increasing {
            true => {
                if curr_elem.le(prev_elem) || !is_safe_change(*prev_elem, *curr_elem) {
                    return false;
                }

                prev_elem = curr_elem;
            }
            false => {
                if curr_elem.ge(prev_elem) || !is_safe_change(*prev_elem, *curr_elem) {
                    return false;
                }

                prev_elem = curr_elem;
            }
        }
    }

    true
}
/**
 * 
 */
fn is_safe_change(x: u64, y: u64) -> bool {
    let lvl_update = x.abs_diff(y);

    return lvl_update >= 1 && lvl_update <= 3;
}
