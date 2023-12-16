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

fn follow_light(data: Vec<Vec<char>>)-> i32{
    let mut q: VecDeque<((i32, i32), Direction)> = VecDeque::new();
    let mut seen: HashSet<((i32, i32), Direction)>= HashSet::new();
    let mut charged: HashSet<(i32, i32)>= HashSet::new();
    q.push_back(((0,0), Direction::Right));
    while let Some(cur) = q.pop_front() {
        println!("{:?}", cur);
        // Check if in range or already seen
        if seen.get(&cur).is_some() || !on_board(cur.0, data.len() as i32, data[0].len() as i32) {
            println!("bad spot");
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
        follow_light(data)
    }
    else {
        panic!("File not found")
    }
}

pub fn p2(_file: &str) -> i32{ 0 }


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
