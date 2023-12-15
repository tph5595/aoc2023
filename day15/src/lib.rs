use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

fn hash_me(s: &String) -> usize{
    let mut current = 0;
    for c in s.chars(){
            current += c as u32;
            current = current * 17;
            current = current % 256;

    }
    current as usize
}

pub fn p1 (file: &str) -> usize {
    if let Ok(lines) = read_lines(file) {
        let data: Vec<String>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .flat_map(|s| s.clone().split(",").map(|i| i.to_owned()).collect::<Vec<String>>())
            .collect();
        return data.iter().map(|s| hash_me(s)).sum();
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
