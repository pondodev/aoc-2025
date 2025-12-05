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
    value:  u64,
}

impl Battery {
    fn from_bank(bank: &String) -> Vec<Self> {
        bank.chars()
            .enumerate()
            .map(|(i, battery)| {
                Self {
                    idx: i,
                    value: battery.to_digit(10).unwrap() as u64
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
        .sum::<u64>();

    println!("t03p1: {}", answer);
}

pub fn p2() {
    let data = read_input_file();

    let answer = data.iter()
        .map(|bank| {
            let batteries = Battery::from_bank(bank);
            let mut valid_batts: Vec<Battery> = vec![];

            const BATTS_NEEDED: usize = 12;
            for i in 1..=BATTS_NEEDED {
                let start_idx = if valid_batts.is_empty() { 0 } else { valid_batts.last().unwrap().idx + 1 };
                let end_buffer = BATTS_NEEDED - i;
                let batt = batteries[start_idx..batteries.len()-end_buffer]
                    .iter()
                    .reduce(|current_highest, batt| {
                        if batt.value > current_highest.value { batt }
                        else { current_highest }
                    })
                    .unwrap();

                valid_batts.push(batt.clone());
            }

            valid_batts
                .iter()
                .fold(0, |acc, b| {
                    acc*10 + b.value
                })
        })
        .sum::<u64>();

    println!("t03p2: {}", answer);
}
