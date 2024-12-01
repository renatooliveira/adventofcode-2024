use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day01.txt").expect("Please create the input file.");

    let lines: Vec<&str> = input.lines().collect();
    let lines2 = lines.clone();
    let lines3 = lines.clone();

    let mut left_column: Vec<i32> = Vec::new();

    let mut right_column: Vec<i32> = Vec::new();

    for line in lines {
        let mut values = line.split("   ");
        let left: i32 = values.next().unwrap().parse().unwrap();
        let right: i32 = values.next().unwrap().parse().unwrap();
        left_column.push(left);
        right_column.push(right);
    }

    left_column.sort();
    right_column.sort();

    let mut sum_distances = 0;
    let mut similarity = 0;

    for (i, _) in lines2.into_iter().enumerate() {
        let mut distance = left_column[i] - right_column[i];
        if distance < 0 {
            distance = distance * -1;
        }

        sum_distances = sum_distances + distance;
    }

    for (i, _) in lines3.into_iter().enumerate() {
        let left = left_column[i];
        let count: i32 = right_column.iter().filter(|&x| *x == left).count() as i32;
        similarity = similarity + (left * count);

    }

    println!("{sum_distances} {similarity}")

}