use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn p1 (file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<String> = lines
            .into_iter()
            .filter_map(|item| item.ok())
            .collect::<String>()
            .split("\n\n")
            .collect();
        println!("{:?}", data);
        return 0;
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
