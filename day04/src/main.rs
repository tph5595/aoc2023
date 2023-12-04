use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::AddAssign;
use std::path::Path;
use std::str::FromStr;
use std::usize;

#[derive(Debug, PartialEq)]
struct Card{
    number: usize,
    winning_numbers: HashSet<usize>,
    found_numbers: Vec<usize>,
}

impl FromStr for Card {

    type Err = ();

    fn from_str(input: &str) -> Result<Card, Self::Err> {

        let mut card_meta= input.split(":");
        let number = card_meta.next().unwrap()
            .split_whitespace()
            .skip(1)
            .next().unwrap()
            .parse().unwrap();
        let mut card = card_meta.next().unwrap().trim().split("|");
        let winning = card.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|i|i.parse().unwrap());
        let found_numbers:Vec<usize>= card.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|i|i.parse().unwrap())
            .collect();

        Ok(Card {number, winning_numbers: HashSet::from_iter(winning), found_numbers})
    }
}

fn calculate_points(c: &Card) -> usize{
    let mut total = 0;
    for n in &c.found_numbers{
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
            .map(|i| calculate_points(&i))
            .sum();
        println!("{:?}", ans);
    }
    else {
        println!("File not found")
    }
}

fn calculate_new_copy(c: &Card) -> usize{
    let mut total = 0;
    for n in &c.found_numbers{
        if c.winning_numbers.contains(&n){
            total += 1;
        }
    }
    total
}

fn calculate_copies(cards: Vec<Card>)-> usize{
    let mut total = 0;
    let mut copies: HashMap<usize, usize>= HashMap::new();
    for card in cards {
            let my_copies = copies.remove(&card.number).unwrap_or(1);
            total += my_copies;
            let p = calculate_new_copy(&card);
            for i in (card.number+1)..=(card.number+p){
                copies.entry(i).or_insert(1).add_assign(my_copies);
            }
    }
    total
}

fn p2(){
    if let Ok(lines) = read_lines("./input.txt") {
        let cards: Vec<Card>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        let ans = calculate_copies(cards);
        println!("{:?}", ans);
    }
    else {
        println!("File not found")
    }
}

fn main() {
    if false{
        p1();
    }
    p2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
