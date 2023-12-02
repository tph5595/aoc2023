use core::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::u32;
// use rayon::prelude::*;

#[derive(Debug, PartialEq)]
struct GameMax {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for GameMax {

    type Err = ();

    fn from_str(input: &str) -> Result<GameMax, Self::Err> {
        let sep = ":";

        let mut data = input.split(sep);

        let id:u32 = data.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for game in  data.last().unwrap().split(";"){
            for pull in game.split(","){
                let mut t = pull.trim().split(" ");
                let number: u32 = t.next().unwrap().parse().unwrap();
                let color = t.last().unwrap();

                match color {
                    "red" => red = cmp::max(red, number),
                    "green" => green = cmp::max(green, number),
                    "blue" => blue = cmp::max(blue, number),
                    _ => panic!("unknown color"),
                }
            }
        }

        Ok(GameMax { 
            id, 
            red, 
            green, 
            blue
        })
    }
}

fn p1(){
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    if let Ok(lines) = read_lines("./input.txt") {
        let data: Vec<GameMax>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        // println!("{:?}", data);
        let ans: u32 = data
            .into_iter()
            .filter(|game| 
                    game.red <= red_max &&
                    game.green <= green_max &&
                    game.blue <= blue_max
                    )
            .map(|game| game.id)
            .sum();
        println!("{:?}", ans);
    }
    else {
        println!("File not found")
    }
}

fn p2(){
    if let Ok(lines) = read_lines("./input.txt") {
        let data: Vec<GameMax>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        // println!("{:?}", data);
        let ans: u32 = data
            .into_iter()
            .map(|game| game.red * game.green * game.blue)
            .sum();
        println!("{:?}", ans);
    }
    else {
        println!("File not found")
    }
}

fn main() {
    p1();
    p2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
