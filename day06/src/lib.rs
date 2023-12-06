use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;

#[derive(Debug, PartialEq)]
pub struct Record{
    time: u32,
    distance: u32,
}

fn number_ways(record: &Record)-> u32{
    // let min = 0;
    // let max_1 = record.time/02.0;
    // let max_2 = ;
    // let max = max_1.
    // // min -> dist = (time-hold)*hold
    // // min -> dist = (time*hold-hold^2
    // // min -> dist = (time-hold)*hold
    // // max -> 0 = (time-hold)
    // min - max
}

pub fn p1 (file: &str) -> u32{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Vec<u32>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.split(":").skip(1).next().unwrap().trim()
                 .split_whitespace().into_iter()
                 .map(|j| j.parse::<u32>().unwrap()).collect())
            .collect();
        let times = data.get(0).unwrap();
        let distances = data.get(1).unwrap();
        let records: Vec<Record> = times.iter().zip(distances.iter())
            .map(|(t,d)| Record {time: *t, distance: *d}).collect();
        let ways = records.iter().map(|r| number_ways(r)).product();
        return ways;
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
