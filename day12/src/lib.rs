use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::{usize, u8};

#[derive(Debug, PartialEq)]
pub struct Row{
    data: String,
    seq: Vec<usize>,
}

impl FromStr for Row {

    type Err = ();

    fn from_str(input: &str) -> Result<Row, Self::Err> {
        let mut data = input.split_whitespace();


        Ok(Row { 
            data: data.next().unwrap()
                .to_owned(), 
            seq: data.next().unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect()
        })
    }
}

fn solve_dp(data: &String, seq: &Vec<usize>) -> usize{
    let cache: HashMap<(usize, usize, usize), usize>= HashMap::new();
    let (ans, _) = dfs(cache, data.as_bytes(), seq,  0, 0, 0);
    ans
}

fn dfs(cache: HashMap<(usize, usize, usize), usize>, 
       data: &[u8], 
       seq: &Vec<usize>, 
       from: usize, 
       group_idx: usize, 
       cur_run: usize) 
    -> (usize, HashMap<(usize, usize, usize), usize>){

    if let Some(ways) = cache.get(&(from, group_idx, cur_run)){
        return (*ways, cache);
    }
    // End of data
    if from >= data.len(){
        // If matched all groups
        if group_idx >= seq.len(){
            return (1, cache);
        }
        // Ended with a match
        if cur_run == seq[group_idx]{
            return (1, cache);
        }
        return (0, cache);

    }
    match data[from]{
        b'.' => {
            // Not tracking anything, just skip '.'s
            if cur_run == 0{

                return dfs(cache, data, seq, from+1, group_idx, cur_run);
            }
            // See if it matched
            if cur_run == seq[group_idx]{
                // match
                let (w, mut cache) = dfs(cache, data, seq, from+1, group_idx+1, 0);
                cache.insert((from, group_idx, cur_run), w);
                return (w, cache);
            }
            return (0, cache);
        },
        b'#' => {
            return dfs(cache, data, seq, from+1, group_idx, cur_run+1);
        },
        // b'?' => {},
        // do two recurse, one where we pretend to add a . the other the #
        _ => unreachable!()
    }
    // return (ways, cache);
}

pub fn p1 (file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Row>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        let ans: usize = data.iter().map(|r| solve_dp(&r.data, &r.seq)).sum();
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
