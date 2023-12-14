use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::usize;

fn test_v(b: &[u8], flip: usize) -> bool{

    let mut l = flip;
    let mut r = flip+1;
    loop {
        if b[l] != b[r]{
            return false;
        }
        // if no more data
        if l == 0 || r == b.len()-1{
            return true;
        }
        l -= 1;
        r += 1;
    } 
}

fn v_search(b: &Vec<String>) -> Option<usize>{
    let mut can: Vec<usize> = (0..b[0].len()-1).collect();
    for r in b {
        if can.len() == 0{
            break;
        }
        can = can.iter()
            .filter(|f| test_v(r.as_bytes(), **f))
            .map(|f| *f)
            .collect();
        
    }
    if can.len() == 1{
        return Some(can[0]+1);
    }
    return None;
}

fn test_h(b: &Vec<String>, col: usize, flip: usize) -> bool{
    // print!("{:?}, ", flip);
    let mut l = flip;
    let mut r = flip+1;
    loop {
        if b[l].as_bytes()[col] != b[r].as_bytes()[col]{
            return false;
        }
        // if no more data
        if l == 0 || r == b.len()-1{
            return true;
        }
        l -= 1;
        r += 1;
    } 
}

fn h_search(b: &Vec<String>) -> Option<usize>{
    let mut can: Vec<usize> = (0..b.len()-1).collect();
    for (i, _) in b[0].chars().enumerate(){
        if can.len() == 0{
            break;
        }
        can = can.iter()
            .filter(|f| test_h(b, i, **f))
            .map(|f| *f)
            .collect();
    }
    // println!("");
    if can.len() == 1{
        return Some(can[0]+1);
    }
    return None;
}

fn fliper(b: &Vec<String>) -> usize{
    if let Some(h_flip) = h_search(b){
        // println!("h: {}", h_flip);
        return h_flip * 100;
    }
     let v = v_search(b).unwrap_or(0);
     // println!("v: {}", v);
     v
}

pub fn p1 (file: &str) -> usize{
    let lines = read_lines(file);
    let data: Vec<Vec<String>> = lines
        .split("\n\n")
        .map(|s| s.to_owned()
             .split("\n")
             .map(|si| si.to_owned())
             .filter(|s| !s.is_empty())
             .collect())
        .collect();
    let ans: usize = data.iter().map(|b| fliper(b)).sum();
    return ans;
}

pub fn p2(_file: &str) -> usize{ 0 }


fn read_lines<P>(filename: P) -> String
where P: AsRef<Path>, {
    let mut data = String::new();
    let f = File::open(filename).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}
