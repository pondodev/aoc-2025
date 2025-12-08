use std::fs::read_to_string;

const FILENAME: &str    = "inputs/t06.txt";

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
struct Equation {
    nums:   Vec<u64>,
    op:     Operation,
}

impl Equation {
    fn new(data: Vec<String>) -> Vec<Self> {
        let mut ops         = vec![];
        let mut num_rows    = vec![];

        for l in data {
            let parts = l.split(" ")
                .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_owned()) })
                .collect::<Vec<String>>();
            if parts[0].parse::<u64>().is_ok() {
                // parse nums
                let nums = parts.into_iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                num_rows.push(nums);
            } else {
                // parse ops
                ops = parts.into_iter()
                    .map(|s| {
                        match s.chars().next().take().unwrap() {
                            '+' => Operation::Add,
                            '*' => Operation::Multiply,
                            _   => panic!("what in tarnation"),
                        }
                    })
                    .collect();
            }
        }

        let mut to_return = vec![];
        for i in 0..ops.len() {
            let nums = num_rows.iter()
                .map(|row| row[i])
                .collect::<Vec<u64>>();
            to_return.push(Self { nums, op: ops[i] });
        }

        to_return
    }

    fn new_again(data: Vec<String>) -> Vec<Self> {
        let ops = data.last().unwrap().chars().collect::<Vec<char>>();
        let num_rows = data[..data.len()-1]
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut to_return = vec![];
        let mut working_equation = Self { nums: vec![], op: Operation::Add };
        for cursor in 0..ops.len() {
            match ops[cursor] {
                '+' => {
                    to_return.push(working_equation.clone());
                    working_equation = Self { nums: vec![], op: Operation::Add };
                },
                '*' => {
                    to_return.push(working_equation.clone());
                    working_equation = Self { nums: vec![], op: Operation::Multiply };
                },
                _   => {},
            }

            let num_res = num_rows.iter()
                .map(|r| r[cursor])
                .collect::<String>()
                .trim()
                .parse::<u64>();

            if let Ok(num) = num_res {
                working_equation.nums.push(num);
            }
        }

        // first equation will be invalid, so we replace it with the trailing equation
        to_return[0] = working_equation;

        to_return
    }
}

pub fn p1() {
    let data = read_input_file();
    let equations = Equation::new(data);

    let answer = equations.into_iter()
        .fold(0, |acc, e| {
            let res = match e.op {
                Operation::Add => e.nums.into_iter().reduce(|total, n| total+n),
                Operation::Multiply => e.nums.into_iter().reduce(|total, n| total*n),
            }.unwrap();

            res+acc
        });

    println!("t06p1: {}", answer);
}

pub fn p2() {
    let data = read_input_file();
    let equations = Equation::new_again(data);

    let answer = equations.into_iter()
        .fold(0, |acc, e| {
            let res = match e.op {
                Operation::Add => e.nums.into_iter().reduce(|total, n| total+n),
                Operation::Multiply => e.nums.into_iter().reduce(|total, n| total*n),
            }.unwrap();

            res+acc
        });

    println!("t06p2: {}", answer);
}
