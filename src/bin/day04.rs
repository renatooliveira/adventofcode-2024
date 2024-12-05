use std::{fs::read_to_string, ops::Index};

fn count_horizontal(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for line in matrix {
        let line_str: String = line.into_iter().collect();
        let count_xmas = line_str.matches("XMAS").count();
        let count_samx = line_str.matches("SAMX").count();
        println!("{line_str} {count_xmas} {count_samx}");

        count += (count_xmas + count_samx) as i32;
    }
    return count;
}

fn count_vertical(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;


    for (line, l) in matrix.iter().enumerate() {
        for (column, c) in l.iter().enumerate() {
            if line < l.len() - 3 {
                if *c == 'X' {

                    if matrix[line + 1][column] == 'M' {
                        if matrix[line + 2][column] == 'A' {
                            if matrix[line + 3][column] == 'S' {
                                println!("{c}, {line}, {column}");
                                count += 1;
                            }
                        }
                    }
                } else if *c == 'S' {
                    if matrix[line + 1][column] == 'A' {
                        if matrix[line + 2][column] == 'M' {
                            if matrix[line + 3][column] == 'X' {
                                println!("{c}, {line}, {column}");
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return count;
}

fn count_diagonal_left(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for (line, l) in matrix.iter().enumerate() {
        for (column, c) in l.iter().enumerate() {
            if line < l.len() - 3 && column < matrix.len() - 3 {
                if *c == 'X' {
                    if matrix[line + 1][column + 1] == 'M' {
                        if matrix[line + 2][column + 2] == 'A' {
                            if matrix[line + 3][column + 3] == 'S' {
                                println!("{c}, {line}, {column}");
                                count += 1;
                            }
                        }
                    }
                } else if *c == 'S' {
                    if matrix[line + 1][column + 1] == 'A' {
                        if matrix[line + 2][column + 2] == 'M' {
                            if matrix[line + 3][column + 3] == 'X' {
                                println!("{c}, {line}, {column}");
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return count;
}

fn count_diagonal_right(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for (line, l) in matrix.iter().enumerate() {
        for (column, c) in l.iter().enumerate() {
            if line < l.len() - 3 && column >= 3 {
                if *c == 'X' {
                    if matrix[line + 1][column - 1] == 'M' {
                        if matrix[line + 2][column - 2] == 'A' {
                            if matrix[line + 3][column - 3] == 'S' {
                                println!("{c}, {line}, {column}");
                                count += 1;
                            }
                        }
                    }
                } else if *c == 'S' {
                    if matrix[line + 1][column - 1] == 'A' {
                        if matrix[line + 2][column - 2] == 'M' {
                            if matrix[line + 3][column - 3] == 'X' {
                                println!("{c}, {line}, {column}");
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return count;
}

fn count_xmas(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for (line, l) in matrix.iter().enumerate() {
        for (column, c) in l.iter().enumerate() {
            if line > 0 && line < l.len() - 1 {
                if column > 0 && column < matrix.len() - 1 {
                    if *c == 'A' {
                        let s: String = [matrix[line - 1][column - 1], matrix[line - 1][column + 1], matrix[line + 1][column - 1], matrix[line + 1][column + 1]].iter().collect();

                        let possibilities = vec!["MSMS", "MMSS", "SSMM", "SMSM"];

                        if possibilities.iter().any(|&x| x == s) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }


    return count;

}

fn main () {
    let input = read_to_string("src/input/day04.txt").expect("Please add the input file");
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count_horizontal = count_horizontal(&matrix);
    let count_vertical = count_vertical(&matrix);
    let count_diagonal_left = count_diagonal_left(&matrix);
    let count_diagonal_right = count_diagonal_right(&matrix);
    let sum = count_horizontal + count_vertical + count_diagonal_left + count_diagonal_right;
    let xmas = count_xmas(&matrix);
    println!("{sum} {xmas}");
}