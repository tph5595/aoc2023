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
    if seq[0] == data.len() as u8{
        return 1;
    }
    if data.iter().all(|c| *c == '?'){
       return data.len() - seq[0] as usize + 1; 
    }
    let mut first = data.len();
    let mut last = 0;
    for (i, c) in data.iter().enumerate(){
        if *c == '#'{
            if i < first{
                first = i;
            }
            if i > last{
                last = i;
            }
        }
    }
    let reserved = last-first;
    if reserved >= seq[0] as usize{
        return 0;
    }
    let n = data.len() - reserved;
    return n.min(first) + n.min(data.len()-last) - (n-1);
}

fn q_ways(data: &[char], seq: &[u8], needed: usize) -> usize{
    let extra = data.len() - needed;
    // println!("{:?}/{:?}/{:?}", data, needed, extra);

    // (seq.len()+1).pow(extra as u32)
    // extra * (seq.len()+1) + 1
    let n = extra+1;
    n*(n+1)/2
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

fn row_perms(data: &[char], seq: &[u8])-> usize{
    // println!("row: {:?}{:?}", data, seq);
    if data.iter().any(|c| *c == '.'){
        // Don't worry I hate myself for this too 
        return solve(&data.iter().collect::<String>().to_owned(), seq);
    }
    if data.is_empty() && seq.is_empty(){
        return 1;
    }
    // Could all be ','
    if data.iter().all(|c| *c == '?') && seq.is_empty(){
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
        // for (j,_) in seq.iter().enumerate(){
        let f = row_perms(&data[..i], &seq[..=0]);
        if f == 0 {
            // println!("bad");
        }
        else{
            // println!("valid: {:?}", f);
            let other = row_perms(&data[(i+1)..], &seq[1..]);
            if other != 0 {
                // println!("found: {:?}", other);
                total += f * other;
                break;
            }
            else {
                // println!("bad");
            }
        }
        // }
    }

    // println!("validd: {:?}", total);
    total
}

fn solve(data_t: &String, seq: &[u8])-> usize{
    let  data = data_t
        .replace("..", ".")
        .trim_start_matches('.')
        .trim_end_matches('.').to_owned();
    if data.is_empty() && seq.is_empty(){
        return 1;
    }
    // Could all be '.'
    if data.chars().all(|c| c == '?' || c == '.') && seq.is_empty(){
        return 1;
    }
    // remove empty sets
    let subproblems: Vec<&str> = data.split('.')
        .filter(|v| !v.is_empty())
        .collect();
    // one problem
    // println!("size: {:?}/{:?}", subproblems, seq);
    if subproblems.len() == 1{
        let d: Vec<char> = data.chars().collect();
        return row_perms(&d[..], seq);
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
        .intersperse_with(||&"." )
        .collect::<String>()
        .replace("..", ".")
        .trim_start_matches('.')
        .trim_end_matches('.')
        .to_owned();

    // println!("sol: {:?}/{:?}", first, rest);
    for (i,_) in data.chars().enumerate().filter(|(_, c)| *c == '.'){
        for (j,_) in seq.iter().enumerate(){
            // Allow for skiping segment
            total += solve(&(data[(i+1)..]).to_owned(), &seq[..]);
            let f = row_perms(&data.chars().collect::<Vec<char>>()[..i], &seq[..=j]);
            if f != 0 {
                total += f * solve(&(data[(i+1)..]).to_owned(), &seq[(j+1)..]);
                // println!("ANS: {:?}", total);
            }
            if total > 0{
                // println!("drop out");
                return total;
            }
        }
        if total > 0{
            // println!("drop out");
            return total;
        }
    }
    // println!("ANS2: {:?}", total);
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
