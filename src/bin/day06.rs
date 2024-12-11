use std::{collections::HashSet, fs::read_to_string};

use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
enum State {
    Walk,
    Turn,
    Finish,
    Loop,
}

#[derive(Debug)]
struct Guard {
    map: &Vec<Vec<char>>,
    initial_position: (i32, i32, Direction),
    current_position: (i32, i32, Direction),
    path: HashSet<(i32, i32, Direction)>,
    direction: Direction,
    is_loop: bool,
    unique_steps: i32,
}

impl Guard {
    fn new(map: &Vec<Vec<char>>, initial_position: (i32, i32, Direction)) -> Guard {
        let direction = Direction::Up;
        let mut path: HashSet<(i32, i32, Direction)> = HashSet::new();
        path.insert(initial_position);
        return Guard {
            map,
            initial_position,
            current_position: initial_position,
            path,
            direction,
            is_loop: false,
            unique_steps: 0,
        }
    }
    fn get_next_state(&self, block: Option<(i32, i32, Direction)>) -> State {
        let mut next_position = (0, 0, Direction::Up);
        match self.direction {
            Direction::Up => {
                next_position = (self.current_position.0, self.current_position.1 - 1, self.direction);
            }
            Direction::Down => {
                next_position = (self.current_position.0, self.current_position.1 + 1, self.direction);
            }
            Direction::Left => {
                next_position = (self.current_position.0 - 1, self.current_position.1, self.direction);
            }
            Direction::Right => {
                next_position = (self.current_position.0 + 1, self.current_position.1, self.direction);
            }
        }

        if let Some(b) = block {
            if b.0 == next_position.0 && b.1 == next_position.1 {
                return State::Turn;
            }
        }

        if self.path.contains(&next_position) {
            return State::Loop;
        } else if self.has_finished(next_position) {
            return State::Finish;
        } else if self.map[next_position.1 as usize][next_position.0 as usize] == '#' {
            return State::Turn;
        } else {
            return State::Walk;
        }
    }

    fn has_finished(&self, position: (i32, i32, Direction)) -> bool {
        return position.0 < 0
            || position.0 >= self.map.len() as i32
            || position.1 < 0
            || position.1 >= self.map[0].len() as i32;
    }
    fn walk(&mut self) {
        match self.direction {
            Direction::Up => {
                self.current_position = (self.current_position.0, self.current_position.1 - 1, self.direction);
            }
            Direction::Down => {
                self.current_position = (self.current_position.0, self.current_position.1 + 1, self.direction);
            }
            Direction::Left => {
                self.current_position = (self.current_position.0 - 1, self.current_position.1, self.direction);
            }
            Direction::Right => {
                self.current_position = (self.current_position.0 + 1, self.current_position.1, self.direction);
            }
        }
        self.path.insert(self.current_position);
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
            }
            Direction::Down => {
                self.direction = Direction::Left;
            }
            Direction::Left => {
                self.direction = Direction::Up;
            }
            Direction::Right => {
                self.direction = Direction::Down;
            }
        }
        self.current_position = (self.current_position.0, self.current_position.1, self.direction);
        self.path.insert(self.current_position);
    }

    fn start(&mut self, block: Option<(i32, i32, Direction)>) {

        loop {
            match self.get_next_state(block) {
                State::Finish => {
                    break;
                }
                State::Turn => {
                    self.turn();
                }
                State::Walk => {
                    self.walk();
                }
                State::Loop => {
                    println!("is loop");
                    self.is_loop = true;
                    break;
                }
            }
        }

        self.unique_steps = self.path.len() as i32;

    }
}


fn would_loop(block: (i32, i32, Direction), initial_position: (i32, i32, Direction), map: &Vec<Vec<char>>) -> bool {

    let mut guard = Guard::new(&map, initial_position);

    if block.0 != initial_position.0 || block.1 != initial_position.1 {
        guard.start(Some(block));
    }
    return guard.is_loop;
}

fn main() {
    let input = read_to_string("src/input/day06.txt").expect("please create the input file");

    let mut initial_position: (i32, i32, Direction) = (0, 0, Direction::Up);
    let mut map: Vec<Vec<char>> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut _line: Vec<char> = Vec::new();
        for (x, _char) in line.chars().enumerate() {
            if _char == '^' {
                initial_position = (x as i32, y as i32, Direction::Up);
                println!("{:?}", initial_position);
                _line.push('.');
            } else {
                _line.push(_char);
            }
        }
        map.push(_line);

    }

    let cloned_map = map.clone();

    let mut guard = Guard::new(&map, initial_position);

    guard.start(None);

    let guards_path = guard.path.clone();

    let mut loops: HashSet<(i32, i32)> = HashSet::new();

    for step in guards_path {
        let would_loop = would_loop(step, initial_position, &cloned_map);
        if would_loop {
            loops.insert((step.0, step.1));
            println!("is renato {:?}", loops.len());
        }
    }

    println!("{:?}", loops.len());
}
