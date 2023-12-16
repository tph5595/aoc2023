use std::collections::{VecDeque, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{i32, usize};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum  Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn add(d: Direction, pos: (i32, i32)) -> ((i32, i32), Direction){
        match d {
            Self::Up => ((pos.0-1, pos.1), Self::Up),
            Self::Down => ((pos.0+1, pos.1), Self::Down),
            Self::Left => ((pos.0, pos.1-1), Self::Left),
            Self::Right => ((pos.0, pos.1+1), Self::Right),
        }
    }
}
fn on_board(cur: (i32, i32), f: i32, s: i32) -> bool{
        (0 <= cur.0 && cur.0 < f) && 
        (0 <= cur.1 && cur.1 < s)
}

fn follow_light(data: &Vec<Vec<char>>, start: ((i32, i32), Direction))-> i32{
    let mut q: VecDeque<((i32, i32), Direction)> = VecDeque::new();
    let mut seen: HashSet<((i32, i32), Direction)>= HashSet::new();
    let mut charged: HashSet<(i32, i32)>= HashSet::new();
    q.push_back(start);
    while let Some(cur) = q.pop_front() {
        // println!("{:?}", cur);
        // Check if in range or already seen
        if seen.get(&cur).is_some() || !on_board(cur.0, data.len() as i32, data[0].len() as i32) {
            // println!("bad spot");
        }
        else {
           // New Space 
           seen.insert(cur.clone());
           charged.insert(cur.0);

           match data[cur.0.0 as usize][cur.0.1 as usize] {
               '.' => {
                   q.push_back(Direction::add(cur.1, cur.0));
               },
               '-' => {
                   match cur.1 {
                       Direction::Up | Direction::Down => {
                           q.push_back(Direction::add(Direction::Left, cur.0));
                           q.push_back(Direction::add(Direction::Right, cur.0));
                       } 
                       Direction::Left | Direction::Right => {
                           q.push_back(Direction::add(cur.1, cur.0));
                       } 
                   }
               },
               '|' => {
                   match cur.1 {
                       Direction::Right | Direction::Left => {
                           q.push_back(Direction::add(Direction::Up, cur.0));
                           q.push_back(Direction::add(Direction::Down, cur.0));
                       }
                       Direction::Up | Direction::Down => {
                           q.push_back(Direction::add(cur.1, cur.0));
                       }
                   }
               },
               '/' => {
                   match cur.1 {
                       Direction::Up => {
                           q.push_back(Direction::add(Direction::Right, cur.0));
                       },
                       Direction::Down => {
                           q.push_back(Direction::add(Direction::Left, cur.0));
                       },
                       Direction::Left => {
                           q.push_back(Direction::add(Direction::Down, cur.0));
                       },
                       Direction::Right => {
                           q.push_back(Direction::add(Direction::Up, cur.0));
                       },
                   }
               },
               '\\' => {
                   match cur.1 {
                       Direction::Up => {
                           q.push_back(Direction::add(Direction::Left, cur.0));
                       },
                       Direction::Down => {
                           q.push_back(Direction::add(Direction::Right, cur.0));
                       },
                       Direction::Left => {
                           q.push_back(Direction::add(Direction::Up, cur.0));
                       },
                       Direction::Right => {
                           q.push_back(Direction::add(Direction::Down, cur.0));
                       },
                   }
               },
               _ => unreachable!()
           }
        }
        
    }
    charged.len() as i32
}

pub fn p1 (file: &str) -> i32{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Vec<char>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.chars().collect())
            .collect();
        follow_light(&data, ((0,0),Direction::Right))
    }
    else {
        panic!("File not found")
    }
}

pub fn p2 (file: &str) -> i32{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Vec<char>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.chars().collect())
            .collect();
        let mut best = 0;
        let start_top = (0..data.len()).map(|i| ((i as i32, 0), Direction::Right));
        let start_bottom = (0..data.len()).map(|i| ((i as i32, (data.len()-1) as i32), Direction::Left));
        let start_left = (0..data[0].len()).map(|i| ((0, i as i32), Direction::Down));
        let start_right = (0..data[0].len()).map(|i| (((data[0].len()-1) as i32, i as i32), Direction::Up));
        let starts = start_top.chain(start_bottom).chain(start_left).chain(start_right);

        for start in starts {
            let new = follow_light(&data, start);
            if new > best{
                best = new;
            }
        }
        best
    }
    else {
        panic!("File not found")
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
