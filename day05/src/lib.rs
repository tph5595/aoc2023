use std::fs::File;
use itertools::izip;
use itertools::Itertools;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::i128;

#[derive(Debug, PartialEq)]
pub struct GardenMap {
    src_start: Vec<i128>,
    dst_start: Vec<i128>,
    total: Vec<i128>,
}

impl FromStr for GardenMap {

    type Err = ();

    fn from_str(input: &str) -> Result<GardenMap, Self::Err> {
        let sep = "\n";
        let lines = input.trim().split(sep).skip(1);

        let mut src_start = Vec::new();
        let mut dst_start = Vec::new();
        let mut total = Vec::new();

        for line in lines {
            // println!("{}", line);
            let values: Vec<i128> = line.split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
            dst_start.push(*values.get(0).unwrap());
            src_start.push(*values.get(1).unwrap());
            total.push(*values.get(2).unwrap());
        }

        Ok(GardenMap { src_start, dst_start, total})
    }
}

fn one_step(current: i128, map: &GardenMap) -> i128 {
    for (src, dst, l) in izip!(&map.src_start, &map.dst_start, &map.total){
        if current >= *src && current <= src + l - 1{
            return current + (dst-src)
        }
    }
    current
}

fn map_seed(seed:i128, maps: &Vec<GardenMap>) -> i128 {
    let mut current = seed;
    for map in maps{
        current = one_step(current, map);
    }
    current
}

pub fn p1 (file: &str) -> i128{
    if let Ok(lines) = read_lines(file) {
        let mut data = lines.into_iter()
            .filter_map(|item| item.ok());
        let seeds: Vec<i128>= data.next().unwrap()
            .split(":").skip(1).next().unwrap()
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let maps: Vec<GardenMap>= data.skip(1)
            .fold(vec!["".to_owned()], |mut groups, val| 
                  {
                      if val == ""{
                          groups.push("".to_owned());
                          groups
                      }else {
                        let new = format!("{}\n{}", groups.pop().unwrap(), val);
                        groups.push(new);
                        groups
                      }
                  })
            .into_iter()
            .map(|i| i.parse().unwrap())
            .collect();
        let lowest = seeds.into_iter().
            map(|s| map_seed(s, &maps)).
            min().unwrap();
        return lowest;
    }
    else {
        panic!("File not found")
    }
}

fn map_seed_range(range_start:i128, range_len: i128, maps: &Vec<GardenMap>) -> i128 {
    let mut current = vec![(range_start, range_len)];
    // let mut count = 0;
    for map in maps{
        current = current.into_iter().flat_map(|i| one_step_range(i, map)).collect();
        // // println!("current: {:?}", current);
        // if count > 3{
        //     // unimplemented!();
        // }
        // count += 1;
    }
    current.into_iter().map(|i| i.0).min().unwrap()
}

fn adjust_range(r1: (i128, i128), r2: (i128, i128)) -> (Vec<(i128, i128)>, Vec<(i128, i128)>) {
    // println!("{:?}\t{:?}", r1, r2);
    if r1.1 < r2.0 || r2.1 < r1.0{
        return (vec![r1], Vec::new());
    }
    let mapped = (r1.0.max(r2.0), r1.1.min(r2.1));
    let mut outstanding = vec![];
    if r1.0 < r2.0{
        let left = (r1.0, r2.0);
        outstanding.push(left);
    }
    if r1.1 > r2.1{
        let right = (r2.1, r1.1);
        outstanding.push(right);
    }
    (outstanding, vec![mapped])
}

fn one_step_range(current: (i128, i128), map: &GardenMap) -> Vec<(i128, i128)>{
    let mut output = Vec::new();
    let mut outstanding = vec![(current.0, current.0 + current.1-1)];
    for (src, dst, l) in izip!(&map.src_start, &map.dst_start, &map.total){
        let mut new_outstanding = vec![];
        for range in outstanding{
            let (mut still_outstanding, mut mapped) = adjust_range(range, (*src, src+l-1));
            new_outstanding.append(&mut still_outstanding);
            mapped = mapped.into_iter().map(|i| (i.0+(dst-src), i.1-i.0+1)).collect();
            output.append(&mut mapped);
        } 
        outstanding = new_outstanding;
    }

    outstanding = outstanding.into_iter().map(|i| (i.0, i.1-i.0+1)).collect();
    output.append(&mut outstanding);
    output
}

fn flip_em(seeds: Vec<i128>, maps: Vec<GardenMap>) -> i128{
    let mut lowest = i128::MAX;

    for chunk in &seeds.into_iter().chunks(2) {
        let seed_range: Vec<i128>= chunk.collect();
        let n = map_seed_range(*seed_range.get(0).unwrap(), *seed_range.get(1).unwrap(), &maps);
        lowest = lowest.min(n);
    }
    lowest
}

pub fn p2(file: &str) -> i128{
    if let Ok(lines) = read_lines(file) {
        let mut data = lines.into_iter()
            .filter_map(|item| item.ok());
        let seeds: Vec<i128>= data.next().unwrap()
            .split(":").skip(1).next().unwrap()
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let maps: Vec<GardenMap>= data.skip(1)
            .fold(vec!["".to_owned()], |mut groups, val| 
                  {
                      if val == ""{
                          groups.push("".to_owned());
                          groups
                      }else {
                        let new = format!("{}\n{}", groups.pop().unwrap(), val);
                        groups.push(new);
                        groups
                      }
                  })
            .into_iter()
            .map(|i| i.parse().unwrap())
            .collect();
        let lowest = flip_em(seeds, maps);
        return lowest;
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
