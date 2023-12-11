use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

fn calc_cols(mut v: Vec<usize>, r: &Vec<char>) -> Vec<usize>{
    for (i, c) in r.iter().enumerate(){
        if v[i] == 1 && *c != '.'{
            v[i] = 0;
        }
    }
    v
}

fn distance(a: &(usize, usize), b: &(usize, usize), rows: &Vec<usize>, cols: &Vec<usize>, expansion: usize) -> usize{
    let r_max = a.0.max(b.0);
    let r_min = a.0.min(b.0);
    let c_max = a.1.max(b.1);
    let c_min = a.1.min(b.1);
    let r_add: usize= rows.iter()
        .filter(|r| **r < r_max && **r > r_min)
        .map(|_| expansion)
        .sum();
    let c_add: usize= cols.iter()
        .filter(|c| **c < c_max && **c > c_min)
        .map(|_| expansion)
        .sum();
    (r_max - r_min) + (c_max - c_min) + r_add + c_add
}

pub fn p1 (file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let map: Vec<Vec<char>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.chars().collect())
            .collect();
        let rows: Vec<usize> = map.iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|c| *c == '.'))
            .map(|(i, _)| i).collect();
        let cols: Vec<usize> = map.iter()
            .fold(vec![1; map[0].len()], |v, r| calc_cols(v, r))
            .iter().enumerate()
            .filter(|(_, col)| **col == 1)
            .map(|(i,_)| i)
            .collect();
        let galaxies: Vec<(usize, usize)>= map.iter().enumerate()
            .flat_map(|(row, v)| 
                      v.iter()
                      .enumerate()
                      .filter(|(_, c)| **c == '#')
                      .map(|(col, _)| (row, col))
                      .collect::<Vec<(usize, usize)>>())
            .collect();
        let ans: usize = galaxies.iter()
            .cartesian_product(&galaxies)
            .filter(|(a,b)| a != b && a > b)
            .map(|(a,b)| distance(a, b, &rows, &cols, 1))
            .sum();
        return ans;
    }
    else {
        panic!("File not found")
    }
}

pub fn p2(file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let map: Vec<Vec<char>>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.chars().collect())
            .collect();
        let rows: Vec<usize> = map.iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|c| *c == '.'))
            .map(|(i, _)| i).collect();
        let cols: Vec<usize> = map.iter()
            .fold(vec![1; map[0].len()], |v, r| calc_cols(v, r))
            .iter().enumerate()
            .filter(|(_, col)| **col == 1)
            .map(|(i,_)| i)
            .collect();
        let galaxies: Vec<(usize, usize)>= map.iter().enumerate()
            .flat_map(|(row, v)| 
                      v.iter()
                      .enumerate()
                      .filter(|(_, c)| **c == '#')
                      .map(|(col, _)| (row, col))
                      .collect::<Vec<(usize, usize)>>())
            .collect();
        let ans: usize = galaxies.iter()
            .cartesian_product(&galaxies)
            .filter(|(a,b)| a != b && a > b)
            .map(|(a,b)| distance(a, b, &rows, &cols, 1000000-1))
            .sum();
        return ans;
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
