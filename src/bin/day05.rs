use std::fs::read_to_string;

use regex::Regex;

fn get_index_in_page(value: &str, page: &Vec<&str>) -> i32 {
    match page.iter().position(|m| m == &value) {
        Some(index) => return index as i32,
        None => return -1,
    }
}

fn is_valid_for_rule(page: &Vec<&str>, rule: (&str, &str)) -> bool {
    let (first, second) = rule;
    let first_index = get_index_in_page(first, page);
    let second_index = get_index_in_page(second, page);
    if first_index >= second_index && first_index != -1 && second_index != -1 {
        return false;
    }

    return true;
}

fn is_valid(page: &Vec<&str>, rules: &Vec<(&str, &str)>) -> bool {
    for rule in rules {
        if is_valid_for_rule(page, *rule) {
            continue;
        }
        return false;
    }
    println!("is valid {:?}", page);
    return true;
}

fn fix_page<'a>(page: &'a Vec<&'a str>, rules: &'a Vec<(&'a str, &'a str)>) -> Vec<&'a str> {
    let mut fixed_for_all = false;
    let mut page_fixed = page.clone();
    println!("fixing page {:?}", page_fixed);
    while !fixed_for_all {
        fixed_for_all = true;
        for rule in rules {
            if !is_valid_for_rule(&page_fixed, *rule) {
                let (first, second) = *rule;
                let first_index = get_index_in_page(first, &page_fixed) as usize;
                let second_index = get_index_in_page(second, &page_fixed) as usize;
                println!(
                    "swapping page {:?} {:?} {first_index} {second_index}",
                    page_fixed, rule
                );
                page_fixed.swap(first_index, second_index);
                fixed_for_all = false;
                break;
            }
        }
    }

    println!("fixed page {:?}", page_fixed);

    return page_fixed;
}

fn main() {
    let input = read_to_string("src/input/day05.txt").expect("please create the input file");
    let input2 = input.clone();

    let rules_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let pages_regex = Regex::new(r"(\d+)(,\d+)+").unwrap();

    let mut sum = 0;
    let mut sum_fixed = 0;

    let rules: Vec<(&str, &str)> = rules_regex
        .captures_iter(&input2)
        .map(|cap| {
            let a = cap.get(1).unwrap().as_str();
            let b = cap.get(2).unwrap().as_str();
            (a, b)
        })
        .collect();
    let pages: Vec<Vec<&str>> = pages_regex
        .captures_iter(&input)
        .map(|cap| cap.get(0).unwrap().as_str().split(",").collect())
        .collect();

    let mut incorrect_pages: Vec<Vec<&str>> = Vec::new();

    for page in &pages {
        let is_valid: bool = is_valid(&page, &rules);
        if is_valid {
            let mid: i32 = page[page.len() / 2].parse().unwrap();
            sum += mid;
        } else {
            incorrect_pages.push(page.clone());
        }
    }

    for page in incorrect_pages {
        let page_fixed = fix_page(&page, &rules);
        let mid: i32 = page_fixed[page_fixed.len() / 2].parse().unwrap();
        sum_fixed += mid;
    }

    println!("{sum} {sum_fixed}");
}
