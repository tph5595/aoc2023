use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand{
    cards: Vec<u32>,
    bet: u32,
    value: u32
}

impl Hand {
    fn calc_value(cards: &Vec<u32>) -> u32{
        let mut map = HashMap::new();
        for c in cards{
            let e = map.entry(c).or_insert(0u32);
            *e += 1;
        }
        let k: Vec<&u32> = map.values().map(|v| v as &u32).collect();
        //Five of a kind, where all five cards have the same label: AAAAA
        if k.len() == 1 {
            return 7;
        }
        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        if k.iter().any(|i| **i == 4){
            return 6;
        }
        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        if k.len() == 2{
            return 5;
        }
        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        if k.iter().any(|i| **i == 3){
            return 4;
        }
        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        if k.len() == 3{
            return 3;
        }
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        if k.len() == 4{
            return 2;
        }
        // High card, where all cards' labels are distinct: 23456
        return 1;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp_result = self.value.cmp(&other.value);
        
        if cmp_result == std::cmp::Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            cmp_result
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand{

    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        let mut data = input.split_whitespace();

        let cards = data.next().unwrap().chars().map(|c|{
            match c {
                'A'=> 14,
                'K'=> 13,
                'Q'=> 12,
                'J'=> 11,
                'T'=> 10,
                _ => c.to_digit(10).unwrap(),
            }
        }).collect();
        let bet: u32 = data.next().unwrap().parse::<u32>().unwrap();
        let value = Hand::calc_value(&cards);

        Ok(Hand{ cards, bet, value})
    }
}

pub fn p1 (file: &str) -> u32{
    if let Ok(lines) = read_lines(file) {
        let mut data: Vec<Hand>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().unwrap())
            .collect();
        data.sort();
        // data.reverse();
        let winnings = data.iter_mut()
            .enumerate()
            .map(|(v, c)| c.bet * ((v+1) as u32)).sum();

        return winnings;
    }
    else {
        panic!("File not found")
    }
}

pub fn p2(_file: &str) -> usize{ 0 }


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
