use std::fs::read_to_string;
use std::cmp::{Ordering, max};

const FILENAME: &str    = "inputs/t05.txt";

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

struct Ingredients {
    id_ranges:  Vec<(u64, u64)>,
    ids:        Vec<u64>,
}

enum ParserState {
    ReadRange,
    ReadID,
}

impl Ingredients {
    fn new(data: Vec<String>) -> Self {
        let mut state                       = ParserState::ReadRange;
        let mut id_ranges: Vec<(u64, u64)>  = vec![];
        let mut ids: Vec<u64>               = vec![];

        for l in data {
            match state {
                ParserState::ReadRange => {
                    if l.trim().is_empty() {
                        state = ParserState::ReadID;
                        continue;
                    }

                    let range = l.split("-")
                        .map(|num_str| num_str.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    id_ranges.push((range[0], range[1]));
                },
                ParserState::ReadID => {
                    ids.push(l.parse::<u64>().unwrap())
                },
            }
        }

        // sort ranges so they're easier to merge in the next step
        id_ranges.sort_by(|a, b| {
            if a.0 < b.0 { Ordering::Less }
            else if a.0 > b.0 { Ordering::Greater }
            else {
                if a.1 < b.1 { Ordering::Less }
                else if a.1 > b.1 { Ordering::Greater }
                else { Ordering::Equal }
            }
        });

        // merge overlapping ranges
        let id_ranges = id_ranges.into_iter()
            .fold(vec![], |mut merged_ranges, range| {
                if merged_ranges.is_empty() {
                    merged_ranges.push(range)
                } else {
                    let last_range = merged_ranges.last_mut().unwrap();
                    // if the next range overlaps with the last range...
                    if range.0 <= last_range.1 {
                        // ...extend the last range to encapsulate the next range...
                        last_range.1 = max(range.1, last_range.1);
                    } else {
                        // ...else push the next range to the vec
                        merged_ranges.push(range);
                    }
                }

                merged_ranges
            });

        Self { id_ranges, ids }
    }
}

fn is_fresh(id: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for r in ranges {
        if id >= r.0 && id <= r.1 { return true; }
    }

    return false;
}

pub fn p1() {
    let data = read_input_file();

    let ingredients = Ingredients::new(data);
    let answer = ingredients.ids.into_iter()
        .filter(|id| is_fresh(*id, &ingredients.id_ranges))
        .collect::<Vec<u64>>()
        .len();

    println!("t05p1: {}", answer);
}

pub fn p2() {
    let data = read_input_file();

    let ingredients = Ingredients::new(data);
    let answer = ingredients.id_ranges.into_iter()
        .fold(0, |acc, range| acc + range.1-range.0+1);

    println!("t05p2: {}", answer);
}
