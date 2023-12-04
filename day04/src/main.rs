use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::usize;

#[derive(Debug, PartialEq)]
struct Card{
    winning_numbers: HashSet<usize>,
    found_numbers: Vec<usize>,
}

impl FromStr for Card {

    type Err = ();

    fn from_str(input: &str) -> Result<Card, Self::Err> {

        let mut card = input.split(":").skip(1).next().unwrap().trim().split("|");
        let winning = card.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|i|i.parse().unwrap());
        let found_numbers:Vec<usize>= card.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|i|i.parse().unwrap())
            .collect();

        Ok(Card {winning_numbers: HashSet::from_iter(winning), found_numbers})
    }
}

fn calculate_points(c: Card) -> usize{
    let mut total = 0;
    for n in c.found_numbers{
        if c.winning_numbers.contains(&n){
            total += 1;
        }
    }
    if total == 0{
        0
    }
    else {
        let base: usize= 2;
        base.checked_pow(total-1).unwrap()
    }
}

fn p1 (){
    if let Ok(lines) = read_lines("./input.txt") {
        let data: Vec<Card>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        let ans: usize= data.into_iter()
            .map(|i| calculate_points(i))
            .sum();
        println!("{:?}", ans);
    }
    else {
        println!("File not found")
    }
}

fn p2(){}

fn main() {
    p1();
    p2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
