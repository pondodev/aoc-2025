use std::fs::read_to_string;

const FILENAME: &str    = "inputs/t01.txt";

const DIAL_MIN: i32     = 0;
const DIAL_MAX: i32     = 99;
const DIAL_START: i32   = 50;

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

pub fn p1() {
    let data = read_input_file();

    let mut dial_pos = DIAL_START;
    let mut pwd = 0;

    for inst in data {
        let dir = inst.chars().next().unwrap();

        let mut delta = inst[1..].parse::<i32>().unwrap();
        if dir == 'L' {
            delta *= -1;
        }

        dial_pos += delta;

        // value wrapping
        while dial_pos < DIAL_MIN {
            dial_pos = DIAL_MAX + (dial_pos+1);
        }
        while dial_pos > DIAL_MAX {
            dial_pos = (dial_pos-1) - DIAL_MAX;
        }

        if dial_pos == 0 {
            pwd += 1;
        }
    }

    println!("t01p1: {}", pwd);
}

pub fn p2() {
    let data = read_input_file();

    let mut dial_pos = DIAL_START;
    let mut pwd = 0;

    for inst in data {
        let dir = if inst.chars().next().unwrap() == 'R' { 1 } else { -1 };
        let delta = inst[1..].parse::<i32>().unwrap();

        // unga bunga it's monkey time
        for _ in 0..delta {
            dial_pos += dir;

            // wrapping
            if dial_pos < DIAL_MIN {
                dial_pos = DIAL_MAX;
            } else if dial_pos > DIAL_MAX {
                dial_pos = DIAL_MIN;
            }

            if dial_pos == 0 {
                pwd += 1;
            }
        }
    }

    println!("t01p2: {}", pwd);
}
