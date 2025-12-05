use std::fs::read_to_string;

const FILENAME: &str    = "inputs/t04.txt";

fn read_input_file() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

struct RollMap {
    width:  isize,
    height: isize,
    tiles:  Vec<char>,
}

impl RollMap {
    fn new(data: Vec<String>) -> Self {
        let width = data[0].len() as isize;
        let height = data.len() as isize;

        let tiles = data
            .into_iter()
            .reduce(|acc, line| {
                format!("{}{}", acc, line)
            })
            .unwrap()
            .chars()
            .collect();

        Self { width, height, tiles }
    }

    fn read_tile(&self, x: isize, y: isize) -> bool {
        let idx = y*self.width + x;
        self.tiles[idx as usize] == '@'
    }

    fn remove_roll(&mut self, x: isize, y: isize) {
        let idx = y*self.width + x;
        self.tiles[idx as usize] = '.';
    }
}

pub fn p1() {
    let data = read_input_file();
    let map = RollMap::new(data);

    let mut answer = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if ! map.read_tile(x, y) { continue }

            let mut found_rolls = 0;
            let search_origin_x = x-1;
            let search_origin_y = y-1;
            for search_offset_y in 0..3 {
                let search_y = search_origin_y+search_offset_y;
                if search_y < 0 || search_y > map.height-1 { continue }

                for search_offset_x in 0..3 {
                    let search_x = search_origin_x+search_offset_x;
                    if search_x < 0 || search_x > map.width-1 { continue }

                    if search_x == x && search_y == y { continue }

                    if map.read_tile(search_x, search_y) { found_rolls += 1 }
                }
            }

            if found_rolls < 4 { answer += 1 }
        }
    }

    println!("t04p1: {}", answer);
}

fn get_accessible_roll_count(mut map: RollMap) -> usize {
    let mut accessible_rolls: Vec<(isize, isize)> = vec![];
    for y in 0..map.height {
        for x in 0..map.width {
            if ! map.read_tile(x, y) { continue }

            let mut found_rolls = 0;
            let search_origin_x = x-1;
            let search_origin_y = y-1;
            for search_offset_y in 0..3 {
                let search_y = search_origin_y+search_offset_y;
                if search_y < 0 || search_y > map.height-1 { continue }

                for search_offset_x in 0..3 {
                    let search_x = search_origin_x+search_offset_x;
                    if search_x < 0 || search_x > map.width-1 { continue }

                    if search_x == x && search_y == y { continue }

                    if map.read_tile(search_x, search_y) { found_rolls += 1 }
                }
            }

            if found_rolls < 4 { accessible_rolls.push((x, y)); }
        }
    }

    for coords in &accessible_rolls {
        map.remove_roll(coords.0, coords.1);
    }

    if accessible_rolls.is_empty() {
        0
    } else {
        accessible_rolls.len() + get_accessible_roll_count(map)
    }
}

pub fn p2() {
    let data = read_input_file();
    let map = RollMap::new(data);

    let answer = get_accessible_roll_count(map);

    println!("t03p2: {}", answer);
}
