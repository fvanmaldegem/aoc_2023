use std::fs;
use std::collections::HashMap;

fn main() {
    let answer1 = solve1();
    let answer2 = solve2();
    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}

fn solve1() -> u32 {
    let input = "input.txt";
    fs::read_to_string(input)
        .expect("Cannot read input")
        .lines()
        .map(|line| -> u32 {
            let nums: Vec<u32> = line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            let first = nums[0];
            let last  = nums[nums.len() - 1];

            format!("{first}{last}").parse().unwrap()
        }).sum()
}

fn solve2() -> u32 {
    let input = "input.txt";
    fs::read_to_string(input)
        .expect("Cannot read input")
        .lines()
        .map(|line| -> u32 {
            let nums: Vec<u32> = convert_line(line)
                .chars()
                .filter_map(|c| c.to_digit(10)).
                collect();
            
            let first = nums[0];
            let last = nums[nums.len() - 1];

            format!("{first}{last}").parse().unwrap()
        }).sum()
}

fn convert_line(line: &str) -> String {
    let mapped_numbers: HashMap<String, String> = HashMap::from([
        (String::from("one"),   String::from("one1one")),
        (String::from("two"),   String::from("two2two")),
        (String::from("three"), String::from("three3three")),
        (String::from("four"),  String::from("four4four")),
        (String::from("five"),  String::from("five5five")),
        (String::from("six"),   String::from("six6six")),
        (String::from("seven"), String::from("seven7seven")),
        (String::from("eight"), String::from("eight8eight")),
        (String::from("nine"),  String::from("nine9nine"))
    ]);

    let mut new_line = String::from(line);

    for (text, num) in &mapped_numbers {
        new_line = new_line.replace(text, &num.to_string());
    }   

    return new_line;
}
