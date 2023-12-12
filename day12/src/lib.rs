use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::{usize, u8, i32, char};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Row{
    data: String,
    seq: Vec<u8>,
}

impl FromStr for Row {

    type Err = ();

    fn from_str(input: &str) -> Result<Row, Self::Err> {
        let mut data = input.split_whitespace();


        Ok(Row { 
            data: data.next().unwrap()
                .replace("..", ".")
                .trim_start_matches('.')
                .trim_end_matches('.')
                .to_owned(), 
            seq: data.next().unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect()
        })
    }
}

fn mixed_single(data: &[char], seq: &[u8]) -> usize{
    if seq[0] + 1 == data.len() as u8{
        return 1;
    }
    return 2;
}

fn q_ways(data: &[char], seq: &[u8], needed: usize) -> usize{
    let extra = data.len() - needed;

    (seq.len()+1).pow(extra as u32)
}

fn valid(data: &[char], seq: &[u8]) -> Option<usize>{
    let good_space:usize = seq.iter().map(|i| *i as i32).sum::<i32>() as usize;
    let dots = seq.len()-1;
    let needed = good_space + dots;
    if needed > data.len() {
        return None;
    }
    return Some(needed);
}

// fn split_seq(data: &[char], seq: &[u8], i: usize) -> usize{
//     0
// }

fn row_perms(data: &[char], seq: &[u8])-> usize{
    println!("{:?}{:?}", data, seq);
    if data.iter().any(|c| *c == '.'){
        unreachable!();
    }
    if data.is_empty() && seq.is_empty(){
        return 1;
    }
    if seq.is_empty(){
        return 0;
    }
    let needed = valid(data, seq);
    // Can't solve from current state; too big
    if needed == None{
        return 0;
    }
    if data.iter().all(|c| *c == '#'){
        if data.len() == seq[0] as usize{
            return 1;
        }
        return 0;
    }
    // Solve only one element
    // assumes no '.' TODO
    if seq.len() == 1{
        return mixed_single(data, seq)
    }
    // Solve all ?'s
    if data.iter().all(|c| *c == '?'){
        return q_ways(data, seq, needed.unwrap());
    }
    // Mixed and multiple:
    // change a ? to a . and recurse the subproblems
    let mut total = 0;
    for (i,_) in data.iter().enumerate().filter(|(_, c)| **c == '?'){
        // split seq
        for (j,_) in seq.iter().enumerate(){
            // let j = split_seq(data, seq, i);
            // let left = row_perms(&data[..i], &seq[..j]);
            // let right = row_perms(&data[i..], &seq[j..]);
            // total += left*right;

            let f = row_perms(&data[..i], &seq[..j]);
            if f == 0 {
                return total;
            }
            total += f * row_perms(&data[i..], &seq[j..])
        }
    }
    total
}

fn solve(data: &String, seq: &[u8])-> usize{
    if data.is_empty() && seq.is_empty(){
        return 1;
    }
    let subproblems: Vec<&str> = data.split('.').collect();
    // one problem
    if subproblems.len() == 1{
        let d: Vec<char> = data.chars().collect();
        row_perms(&d[..], seq);
    }
    // exact fit
    if subproblems.len() == seq.len(){
        return subproblems.iter()
            .zip(seq)
            .map(|(di, s)| row_perms(&di.chars().collect::<Vec<char>>()[..]
                                     , &[*s]) as i32).product::<i32>() as usize;
    }
    // One at a time
    let mut total = 0;
    let mut d_iter = data.split('.');
    let first = d_iter.next().unwrap();
    let rest: String= d_iter
        .filter(|s| !s.is_empty())
        .intersperse_with(||&"." ).collect();

    println!("{:?}{:?}", first, rest);
    for (i,_) in seq.iter().enumerate(){
        let f = row_perms(&first.chars().collect::<Vec<char>>()[..], &seq[..=i]);
        if f == 0 {
            println!("ANS: {:?}", total);
            return total;
        }
            total += f * solve(&rest, &seq[(i+1)..])
    }
    println!("ANS2: {:?}", total);
    total
}

pub fn p1 (file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Row>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        let ans: usize = data.iter().map(|r| solve(&r.data, &r.seq[..])).sum();
        return ans;
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
