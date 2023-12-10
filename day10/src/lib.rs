use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::u128;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn advance(pos: (usize, usize), d: &Direction) -> (usize,usize){
    match d{
        Direction::North => (pos.0 - 1, pos.1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::West => (pos.0, pos.1 - 1),
    }
}

fn follow_map(map: Vec<Vec<char>>, start:(usize,usize)) -> u128{
    let mut l = 1;
    let mut pos = start;
    let mut last_dir = Direction::East;

    // Start
    pos = advance(pos, &last_dir);
    
    loop {
        let c = map.get(pos.0).unwrap().get(pos.1).unwrap();
        if *c == 'S'{
            break
        }
        last_dir = match (*c, &last_dir){
            // Same Dir 
            ('|', Direction::North) => Direction::North,
            ('|', Direction::South) => Direction::South,
            ('-', Direction::East) => Direction::East,
            ('-', Direction::West) => Direction::West,
            // Flip Dir
            ('L', Direction::South) => Direction::East,
            ('L', Direction::West) => Direction::North,

            ('J', Direction::South) => Direction::West,
            ('J', Direction::East) => Direction::North,

            ('7', Direction::North) => Direction::West,
            ('7', Direction::East) => Direction::South,

            ('F', Direction::North) => Direction::East,
            ('F', Direction::West) => Direction::South,
            _ => unreachable!("{:?}{:?}", pos, last_dir)
        };
        l += 1;
        pos = advance(pos, &last_dir);
    }

    l
}

pub fn p1 (file: &str) -> u128{
    if let Ok(lines) = read_lines(file) {
        let map: Vec<Vec<char>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|s| s.chars().collect())
            .collect();
        let starts: Vec<(usize, usize)>= map.iter()
            .enumerate()
            .map(|(row, s)| 
                    (row,
                     s.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'S')
                    .collect::<Vec<(usize, &char)>>()))
            .filter(|(_, v)| !v.is_empty())
            .map(|(row, v)| (row, v.get(0).unwrap().0))
            .collect();
        let l = follow_map(map, *starts.get(0).unwrap());
        return l/2;
    }
    else {
        panic!("File not found")
    }
}

fn mark_map(mut map: Vec<Vec<char>>, start:(usize,usize)) -> Vec<Vec<char>>{
    let mut pos = start;
    let mut last_dir = Direction::East;

    // Start
    pos = advance(pos, &last_dir);
    
    loop {
        let c = *map.get(pos.0).unwrap().get(pos.1).unwrap();
        if c == 'S'{
            break
        }
        // mark 
        match last_dir {
            Direction::North=> if map[pos.0][pos.1+1] == '.'{
                map[pos.0][pos.1+1]  = 'G';
            },
            Direction::East=>  if map[pos.0+1][pos.1] == '.' {
                map[pos.0+1][pos.1] = 'G';
            },
            Direction::South=> if map[pos.0][pos.1-1] == '.'{
                map[pos.0][pos.1-1]  = 'G';
            },
            Direction::West=>  if map[pos.0-1][pos.1] == '.' {
                map[pos.0-1][pos.1]  = 'G';
            },
        };
        // Move
        last_dir = match (c, &last_dir){
            // Same Dir 
            ('|', Direction::North) => Direction::North,
            ('|', Direction::South) => Direction::South,
            ('-', Direction::East) => Direction::East,
            ('-', Direction::West) => Direction::West,
            // Flip Dir
            ('L', Direction::South) => Direction::East,
            ('L', Direction::West) => Direction::North,

            ('J', Direction::South) => Direction::West,
            ('J', Direction::East) => Direction::North,

            ('7', Direction::North) => Direction::West,
            ('7', Direction::East) => Direction::South,

            ('F', Direction::North) => Direction::East,
            ('F', Direction::West) => Direction::South,
            _ => unreachable!("{:?}{:?}", pos, last_dir)
        };
        pos = advance(pos, &last_dir);
    }
    map
}

fn calc_inside(mut map: Vec<Vec<char>>) -> u128{
    let mut inside = 0;
    for (i, row) in map.clone().iter().enumerate(){
        for (j, c) in row.iter().enumerate(){
            if *c == 'G'{
                inside += 1;
            }
            if *c == '.' && i > 0 && j > 0 && i < map.len()-1 && j < map[i].len()-1{
                if map[i-1][j] == 'G' ||
                   map[i+1][j] == 'G' ||
                   map[i][j-1] == 'G' || 
                   map[i][j+1] == 'G'{
                    inside += 1;
                    map[i][j] = 'G';
                }
            }
        }
    }
    inside
}

pub fn p2(file: &str) -> u128{
    if let Ok(lines) = read_lines(file) {
        let map: Vec<Vec<char>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|s| s.chars().collect())
            .collect();
        let starts: Vec<(usize, usize)>= map.iter()
            .enumerate()
            .map(|(row, s)| 
                    (row,
                     s.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'S')
                    .collect::<Vec<(usize, &char)>>()))
            .filter(|(_, v)| !v.is_empty())
            .map(|(row, v)| (row, v.get(0).unwrap().0))
            .collect();
        let marked_map = mark_map(map, *starts.get(0).unwrap());
        println!("{:?}", marked_map);
        let mut  ans = calc_inside(marked_map.clone());
        loop{
            let t = calc_inside(marked_map.clone());
            if t == ans{
                break;
            }
            ans = t;
        }
        return ans;
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