use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct IP {
    first: u8,
    second: u8,
    third: u8,
    fourth: u8,
}

impl FromStr for IP {

    type Err = ();

    fn from_str(input: &str) -> Result<IP, Self::Err> {
        let sep = ".";

        let mut data = input.split(sep);

        Ok(IP{
            first: data.next().unwrap_or_default().parse().unwrap(),
            second: data.next().unwrap_or_default().parse().unwrap(),
            third: data.next().unwrap_or_default().parse().unwrap(),
            fourth: data.next().unwrap_or_default().parse().unwrap(),
        })
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let ip:IP = ip.parse().unwrap();
                println!("{:?}", ip);
            }
        }
    }
    else {
        println!("File not found")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
