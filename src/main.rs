mod aoc_utils;

use std::fs::read_to_string;

fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============

    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", total);
    total
}



fn main() {
    let filename = "input.txt".to_string();
    let day = 6; // TO MODIFY
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = aoc_utils::dl_file_from_url(input_url);

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = aoc_utils::upload_solution(year, day, 1, p1_res.to_string());
    let _ = aoc_utils::upload_solution(year, day, 2, p2_res.to_string());
    let output = read_to_string("output.txt").expect("Something went wrong reading the file");
    println!("{}", output);
}
