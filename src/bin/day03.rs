use std::fs::read_to_string;

use regex::Regex;

fn evaluate_line(line: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let results: Vec<i32> = re.captures_iter(line).map(
        |m| {
            let first: i32 = m[1].parse().unwrap();
            let second: i32 = m[2].parse().unwrap();
            first * second
        })
        .collect();

    return results.iter().sum();
}

fn remove_disabled(line: &str) -> String {
    let re = Regex::new(r"(?s)don't\(\)(.*?)(do\(\)|$)").unwrap();
    for m in re.captures_iter(line) {
        let x = &m[0];
        println!("{x}")
    }
    return re.replace_all(line, "").to_string();
}

fn main () {

    let input = read_to_string("src/input/day03.txt").expect("Please add the input file");

    let _line = remove_disabled(&input);
    println!("{_line}");
    let res = evaluate_line(&_line);

    println!("{res}");

}