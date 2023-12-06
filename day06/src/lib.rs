use std::fs::File;
use std::f64;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Record{
    time: f64,
    distance: f64,
}

    // let main = ((b.powi(2)-4.0*a*c)/(2.0*a)).sqrt();
fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64){
    let mut top = b.powi(2)-4.0*a*c;
    top = top.sqrt();
    // println!("{:?}", top);
    let bottom = 2.0*a;
    // println!("{:?}", bottom);
    // println!("{:?}", main);
    ((-b+top)/bottom, (-b-top)/bottom)
}

fn number_ways(record: &Record)-> f64{
    // dist = (time-hold)*hold
    // dist = time*hold-hold^2
    let (max, min) = quadratic_formula(1.0, -1.0*record.time, record.distance);
    (max.floor() - min.ceil())+1.0
}

pub fn p1 (file: &str) -> f64{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Vec<f64>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.split(":").skip(1).next().unwrap().trim()
                 .split_whitespace().into_iter()
                 .map(|j| j.parse::<f64>().unwrap()).collect())
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

pub fn p2(file: &str) -> f64{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<f64>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.split(":").skip(1).next().unwrap().trim()
                 .replace(" ", "").parse().unwrap())
            .collect();
        let time = *data.get(0).unwrap();
        let distance = *data.get(1).unwrap();
        let record = Record {time, distance};
        let ways = number_ways(&record);
        return ways;
    }
    else {
        panic!("File not found")
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
