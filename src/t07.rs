use std::fs::read_to_string;

const FILENAME: &str    = "inputs/t07.txt";

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

struct Diagram {
    width:  usize,
    height: usize,
    map:    Vec<char>,
}

impl Diagram {
    fn new(data: Vec<String>) -> Self {
        let width   = data[0].len();
        let height  = data.len();

        let map = data.into_iter()
            .reduce(|acc, l| acc+&l)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        Self { width, height, map }
    }

    fn read_tile(&self, x: usize, y: usize) -> char {
        let idx = y*self.width + x;
        self.map[idx]
    }

    fn get_start_coord(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.read_tile(x, y) == 'S' {
                    return (x, y)
                }
            }
        }

        panic!("ruh roh raggy");
    }

    fn check_splitter(&self, x: usize, y: usize) -> bool {
        self.read_tile(x, y) == '^'
    }
}

pub fn p1() {
    let data = read_input_file();
    let diagram = Diagram::new(data);

    let mut lasers = vec![diagram.get_start_coord()];
    let mut answer = 0;
    for y in 0..diagram.height {
        // adjust the y pos of each laser
        lasers = lasers.iter().map(|l| (l.0, y)).collect();

        // check if any laser is on a splitter. if so, split it
        let mut new_lasers: Vec<(usize, usize)> = vec![];
        for l in &mut lasers {
            if ! diagram.check_splitter(l.0, l.1) {
                if ! new_lasers.iter().any(|nl| nl.0 == l.0 && nl.1 == l.1) {
                    new_lasers.push(l.clone());
                }
                continue
            }

            answer += 1;
            let left_laser = if l.0 != 0 { Some((l.0-1, l.1)) } else { None };
            let right_laser = if l.0 != diagram.width-1 { Some((l.0+1, l.1)) } else { None };

            // ensure we don't add duplicates
            if let Some(left) = left_laser {
                if ! new_lasers.iter().any(|nl| nl.0 == left.0 && nl.1 == left.1) {
                    new_lasers.push(left);
                }
            }
            if let Some(right) = right_laser {
                if ! new_lasers.iter().any(|nl| nl.0 == right.0 && nl.1 == right.1) {
                    new_lasers.push(right);
                }
            }

        }

        lasers = new_lasers
    }

    println!("t07p1: {}", answer);
}

pub fn p2() {
    let data = read_input_file();

    let answer = 0;

    println!("t07p2: {}", answer);
}
