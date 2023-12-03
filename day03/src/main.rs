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

#[derive(Debug, PartialEq)]
struct Symbol {
    loc: (usize, usize),
}

#[derive(Debug, PartialEq)]
enum Data {
    N(Number),
    S(Symbol)
    
}

fn parse(lines: Vec<String>) -> Vec<Data>{
    let mut values = Vec::new();
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
                        values.push(Data::N(Number { value: current as usize, 
                            start: start.unwrap(), 
                            end: (col-1, row)
                        }))
                    }
                    current = 0;
                    start = None;
                },
                _ => {
                    values.push(Data::S(Symbol { loc: (col,row) }));
                    if start != None && col != 0{
                        values.push(Data::N(Number { value: current as usize, 
                            start: start.unwrap(), 
                            end: (col-1, row)
                        }))
                    }
                    current = 0;
                    start = None;
                },
            };
        }
    }
    values
}

fn p1 (){
    if let Ok(lines) = read_lines("./test.txt") {
        let raw_data: Vec<String>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .collect();
        let data = parse(raw_data);
        println!("{:?}", data);
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
