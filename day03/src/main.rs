use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::usize;

#[derive(Debug, PartialEq)]
struct Data<'a>{
    values: &'a mut Vec<Number>,
}

impl Data<'_>{
    fn setrow(mut self, row: usize) -> Self{
        for n in self.values{
            n.start.1 = row;
            n.end.1 = row;
        }
        self
    }
}

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


impl<'a> FromStr for Data<'a>{

    type Err = ();

    fn from_str(input: &str) -> Result<Data<'a>, Self::Err> {
        let mut values = Vec::new();
        let mut current = 0;
        let mut start = 99999;
        for (i, c) in input.chars().enumerate(){
            match c{
                '0'..='9'=> {
                    current = current*10 + c.to_digit(10).unwrap();
                    if start == 99999{
                        start = i
                    }
                },
                '.' => {
                    if start != 99999{
                        values.push(Number { value: current as usize, start: (start,0), end: (i-1,0) })

                    }
                    current = 0;
                    start = 99999;
                },
                _ => {},
            };
        }

        Ok(Data { values })
    }
}

fn p1 (){
    if let Ok(lines) = read_lines("./input.txt") {
        let data: Vec<Number>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .enumerate()
            .map(|(i, line)| line.parse::<Data>().unwrap().setrow(i))
            // .flat_map(|i| i.values)
            .collect();
        println!("{:?}", data);
    }
    else {
        println!("File not found")
    }
}

// fn p2(){}

fn main() {
    p1();
    // p2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
