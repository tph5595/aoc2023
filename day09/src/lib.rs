use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::i128;

fn next_val(mut values: Vec<i128>) -> i128{
    let mut s: i128 = 0;
    loop {
        if values.iter().all(|x| *x == 0){
            break;
        }
        s += values.last().unwrap();
        values = values
            .windows(2)
            .map(|w| 
                 w.get(1).unwrap() - w.get(0).unwrap() )
            .collect();
    }
    s
}

pub fn p1 (file: &str) -> i128{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Vec<i128>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| {
                i.split_whitespace().map(|i| i.parse().unwrap()).collect()
            })
            .collect();
            let next: Vec<i128> = data.into_iter()
                .map(|v| next_val(v))
                .collect();
        return next.into_iter().sum();
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
