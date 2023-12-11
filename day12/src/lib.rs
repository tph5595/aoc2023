use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct IP {
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

pub fn p1 (file: &str) -> Vec<IP>{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<IP>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        return data;
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
