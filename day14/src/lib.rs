use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

fn tilt_north(data: &mut Vec<Vec<u8>>){
    let mut next_spot: Option<usize>;
    for col in 0..data.get(0).unwrap().len(){
        next_spot = None;
        for row in 0..data.len(){
            match data[row][col] {
                b'.' => {
                    if next_spot == None{
                        next_spot = Some(row);
                    }
                },
                b'#' => {
                    next_spot = None;
                },
                b'O' => {
                    // If None then it cannot move
                    if next_spot != None{
                        // erase from current spot 
                        data[row][col] = b'.';
                        // put in new spot 
                        data[next_spot.unwrap()][col] = b'O';
                        // look for next place
                        next_spot = Some(next_spot.unwrap() + 1);
                    }
                },
                _ => unreachable!()
            }
        }
    }
}

fn byte_to_strings(data: &Vec<Vec<u8>>)-> Vec<String>{
    data
        .iter()
        .map(|bytes| String::from_utf8_lossy(bytes).to_string())
        .collect()
}

pub fn p1 (file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let mut data: Vec<Vec<u8>> = lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|s| s.as_bytes().to_vec())
            .collect();
        // println!("{:?}", byte_to_strings(&data));
        tilt_north(&mut data);
        // println!("{:?}", byte_to_strings(&data));
        let data_size = data.len();
        let ans: usize = data.iter()
            .enumerate()
            .map(|(i, r)| {
                let Os: usize = r.iter()
                    .filter(|c| **c == b'O')
                    .map(|_| 1)
                    .sum();
                Os * (data_size-i)
            }
            ).sum();
        return ans;
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
