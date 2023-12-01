use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{u32, u128};

fn p1(){
    if let Ok(lines) = read_lines("./input.txt") {
        let data: Vec<u32>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.chars().filter(|c| c.is_digit(10)).collect())
            .map(|i:String| 
                 format!("{}{}", 
                 i.chars().nth(0).unwrap(),
                 i.chars().nth(i.len()-1).unwrap()))
            .map(|i| i.parse().unwrap())
            .collect();
        let ans: u32 = data.iter().sum();
        println!("{:?}", ans);
    }
    else {
        println!("File not found")
    }
}

fn new_numbers(mut s:String, words: &Vec<(&str, &str)>) -> String{
    for w in words{
        s = s.replace(w.0, w.1)
    }
    s
}

fn p2(){
    let words: Vec<(&str, &str)> = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    
    if let Ok(lines) = read_lines("./input.txt") {
        let data: Vec<u128>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|s| new_numbers(s, &words))
            .map(|i| i.chars().filter(|c| c.is_digit(10)).collect())
            .map(|i:String| 
                 format!("{}{}", 
                 i.chars().nth(0).unwrap(),
                 i.chars().nth(i.len()-1).unwrap()))
            .map(|i| i.parse().unwrap())
            .collect();
        let ans: u128 = data.iter().sum();
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
