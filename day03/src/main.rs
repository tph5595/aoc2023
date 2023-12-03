use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

#[derive(Debug, PartialEq)]
struct Number{
    value: usize,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse(lines: Vec<String>) -> (Vec<Number>, HashSet<(usize,usize)>){
    let mut n = Vec::new();
    let mut s = HashSet::new();
    let mut current = 0;
    let mut start = None;
    for (row, l) in lines.iter().enumerate(){
        for (col, c) in l.chars().enumerate(){
            match c{
                '0'..='9'=> {
                    current = current*10 + c.to_digit(10).unwrap();
                    if start == None{
                        start = Some((col, row))
                    }
                },
                '.' => {
                    if start != None && col != 0{
                        n.push(Number { value: current as usize, 
                            start: start.unwrap(), 
                            end: (col-1, row)
                        })
                    }
                    current = 0;
                    start = None;
                },
                _ => {
                    s.insert((col,row));
                    if start != None && col != 0{
                        n.push(Number { value: current as usize, 
                            start: start.unwrap(), 
                            end: (col-1, row)
                        })
                    }
                    current = 0;
                    start = None;
                },
            };
        }
        if start != None{
            n.push(Number { value: current as usize, 
                start: start.unwrap(), 
                end: (l.len(), row)
            })
        }
    }
    (n,s)
}

fn gen_set(n: &Number) -> HashSet<(usize, usize)>{
    let mut n_set = HashSet::new();
    
    for r in n.start.1.saturating_sub(1)..=n.end.1.saturating_add(1){
        for c in n.start.0.saturating_sub(1)..=n.end.0.saturating_add(1){
            n_set.insert((c,r));
        }
    }
    n_set
}

fn find_good(numbers: Vec<Number>, symbols: HashSet<(usize, usize)>)-> Vec<Number>{
    let mut valid = Vec::new();
    for n in numbers{
        let n_set = gen_set(&n);
        if !symbols.is_disjoint(&n_set){
            valid.push(n);
        }
    }
    valid
}

fn p1 (){
    if let Ok(lines) = read_lines("./input.txt") {
        let raw_data: Vec<String>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .collect();
        let (numbers, symbols) = parse(raw_data);
        let good_numbers = find_good(numbers, symbols);

        let s: usize = good_numbers.iter().map(|i| i.value).sum();
        let nums: Vec<usize> = good_numbers.iter().map(|i| i.value).collect();
        
        println!("{:?}", nums);
        println!("{:?}", s);
    }
    else {
        println!("File not found")
    }
}


fn main() {
    p1();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
