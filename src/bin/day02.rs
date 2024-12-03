use std::fs::read_to_string;

fn is_ascending(line: &Vec<&str>) -> bool {
    let mut is_safe: bool = true;
    let mut is_first: bool = true;
    let mut num = 0;
    for item in line.iter() {
        let value: i32 = item.parse().expect("this should be an integer");
        if !is_first {
            if (value - num >= 1) && (value - num <= 3) {
                num = value;
                continue;
            } else {
                is_safe = false;
                break;
            }
        } else {
            num = value;
            is_first = false;
        }
    }
    return is_safe;
}

fn is_descending(line: &Vec<&str>) -> bool {
    let mut is_safe: bool = true;
    let mut is_first: bool = true;
    let mut num = 0;
    for item in line.iter() {
        let value: i32 = item.parse().expect("this should be an integer");

        if !is_first {
            if (value - num <= -1) && (value - num >= -3) {
                num = value;
                continue;
            } else {
                is_safe = false;
                break;
            }
        } else {
            num = value;
            is_first = false;
        }
    }

    return is_safe;
}

fn get_total_safe_levels(levels: &Vec<&str>) -> i32 {
    let mut total_safe = 0;

    for level in levels {
        let line = *level;
        let values: Vec<&str> = line.split(" ").collect();
        let mut is_asc = is_ascending(&values);
        let mut is_desc = is_descending(&values);
        if is_asc || is_desc {
            total_safe += 1;
        } else {
            for (i, _) in values.iter().enumerate() {
                let mut new_values = values.clone();
                new_values.remove(i);
                is_asc = is_asc || is_ascending(&new_values);
                is_desc = is_desc || is_descending(&new_values);
                if is_asc || is_desc {
                    total_safe += 1;
                    break;
                }
            }

        }
    }
    return total_safe;
}

fn main() {
    let input = read_to_string("src/input/day02.txt").expect("Please create the input file.");

    let levels: Vec<&str> = input.lines().collect();

    let safe_levels: i32 = get_total_safe_levels(&levels);

    println!("number of safe levels: {safe_levels}");
}
