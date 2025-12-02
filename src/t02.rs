use std::fs::read_to_string;

const FILENAME: &str    = "inputs/t02.txt";

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result
            .extend(
                line
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            );
    }

    result
}

struct IDRange {
    begin:  usize,
    end:    usize,
}

impl IDRange {
    fn new(range_str: String) -> Self {
        let limits: Vec<usize> =
            range_str
                .split("-")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

        Self {
            begin:  limits[0],
            end:    limits[1],
        }
    }
}

pub fn p1() {
    let data = read_input_file();
    let mut answer = 0;

    for r in data {
        let range = IDRange::new(r);
        for i in range.begin..=range.end {
            let num_str = i.to_string();
            if num_str.len() % 2 != 0 { continue; }

            let half_point = num_str.len() / 2;
            let first_half = num_str[..half_point].to_owned();
            let second_half = num_str[half_point..].to_owned();

            if first_half == second_half {
                answer += i;
            }
        }
    }

    println!("t02p1: {}", answer);
}

pub fn p2() {
    let data = read_input_file();
    let mut answer = 0;

    for r in data {
        let range = IDRange::new(r);
        answer += (range.begin..=range.end)
            .into_iter()
            .filter(|i| {
                let num_str = i.to_string();
                let token_size_limit = num_str.len() / 2;

                for token_size in 1..=token_size_limit {
                    if num_str.len() % token_size != 0 { continue; }

                    let subs = num_str.as_bytes()
                        .chunks(token_size)
                        .map(|buf| str::from_utf8(buf).unwrap())
                        .collect::<Vec<&str>>();

                    let first = subs[0];
                    if subs.iter().all(|&x| x == first) {
                        return true
                    }
                }

                false
            })
            .sum::<usize>();
    }

    println!("t02p2: {}", answer);
}
