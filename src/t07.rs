use std::fs::read_to_string;
use std::rc::Rc;
use std::cell::RefCell;

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

struct GraphNode {
    pos:        (usize, usize),
    children:   Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    fn new(pos: (usize, usize)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { pos, children: vec![] }))
    }
}

struct Laser {
    pos:    (usize, usize),
    origin: Rc<RefCell<GraphNode>>,
}

impl Laser {
    fn new(pos: (usize, usize), origin: Rc<RefCell<GraphNode>>) -> Self {
        Self { pos, origin }
    }
}

pub fn p2() {
    let data = read_input_file();
    let diagram = Diagram::new(data);

    let answer = 0;
    let start = diagram.get_start_coord();
    let root = GraphNode::new(start.clone());

    let mut lasers = vec![Laser::new(start, Rc::clone(&root))];
    for y in 0..diagram.height {
        println!("now at y: {}", y);
        println!("laser count: {}", lasers.len());
        // adjust the y pos of every laser
        lasers = lasers.iter()
            .map(|l| Laser::new((l.pos.0, y), Rc::clone(&l.origin)))
            .collect();

        // check if any lasers are on a splitter. if so, split it and update
        // the tree
        let mut new_lasers = vec![];
        for l in lasers {
            if ! diagram.check_splitter(l.pos.0, l.pos.1) {
                new_lasers.push(l);
                continue
            }

            // we're on a splitter, so now we need to:
            // 1. create a new node
            // 2. add the node as a child of the source node the laser came from
            // 3. create two new lasers with with the new node as the origin
            let new_node = GraphNode::new(l.pos);
            l.origin.borrow_mut().children.push(Rc::clone(&new_node));

            let left_laser = if l.pos.0 != 0 {
                Some(Laser::new((l.pos.0-1, l.pos.1), Rc::clone(&new_node)))
            } else {
                None
            };
            let right_laser = if l.pos.0 != diagram.width-1 {
                Some(Laser::new((l.pos.0+1, l.pos.1), Rc::clone(&new_node)))
            } else {
                None
            };

            if let Some(left) = left_laser { new_lasers.push(left) }
            if let Some(right) = right_laser { new_lasers.push(right) }
        }

        lasers = new_lasers
    }

    println!("t07p2: {}", answer);
}
