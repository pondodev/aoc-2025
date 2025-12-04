use std::fs::read_to_string;

const FILENAME: &str    = "inputs/t03.txt";

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Battery {
    idx:    usize,
    value:  u32,
}

impl Battery {
    fn from_bank(bank: &String) -> Vec<Self> {
        bank.chars()
            .enumerate()
            .map(|(i, battery)| {
                Self {
                    idx: i,
                    value: battery.to_digit(10).unwrap()
                }
            })
            .collect()
    }
}

pub fn p1() {
    let data = read_input_file();

    let answer = data.iter()
        .map(|bank| {
            let batteries = Battery::from_bank(bank);

            let left_batt = batteries[..batteries.len()-1]
                .iter()
                .reduce(|current_highest, batt| {
                    if batt.value > current_highest.value { batt }
                    else { current_highest }
                })
                .unwrap();
            let right_batt = batteries[left_batt.idx+1..]
                .iter()
                .reduce(|current_highest, batt| {
                    if batt.value > current_highest.value { batt }
                    else { current_highest }
                })
                .unwrap();

            left_batt.value*10 + right_batt.value
        })
        .sum::<u32>();

    println!("t03p1: {}", answer);
}

pub fn p2() {
    let data = read_input_file();
    let mut answer = 0;

    println!("t03p2: {}", answer);
}
