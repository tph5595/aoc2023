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
    let mut cache: HashMap<(usize, usize, usize), usize>= HashMap::new();
    dfs(&mut cache, data.as_bytes(), seq,  0, 0, 0)
}

fn dfs(cache: &mut HashMap<(usize, usize, usize), usize>, 
       data: &[u8], 
       seq: &Vec<usize>, 
       from: usize, 
       group_idx: usize, 
       cur_run: usize) 
    -> usize {

    if let Some(ways) = cache.get(&(from, group_idx, cur_run)){
        return *ways;
    }
    // End of data
    if from >= data.len(){
        // If matched all groups
        if group_idx >= seq.len(){
            return 1;
        }
        // Ended with the final match
        if cur_run == seq[group_idx] && group_idx == seq.len() - 1 {
            return 1;
        }
        return 0;

    }
    match data[from]{
        b'.' => {
            // Not tracking anything, just skip '.'s
            if cur_run == 0{
                return dfs(cache, data, seq, from+1, group_idx, cur_run);
            }
            // See if it matched
            if group_idx < seq.len() && cur_run == seq[group_idx]{
                // match
                let w = dfs(cache, data, seq, from+1, group_idx+1, 0);
                // cache.insert((from, group_idx, cur_run), w);
                return w;
            }
            return 0;
        },
        b'#' => {
            // if too full
            if group_idx >= seq.len() || seq[group_idx] < cur_run+1{
                return 0;
            }
            return dfs(cache, data, seq, from+1, group_idx, cur_run+1);
        },
        // do two recurse, one where we pretend to add a . the other the #
        b'?' => {
            let mut ways = 0;
            // If nothing else to match, treat as '.'
            if group_idx >= seq.len(){
                return dfs(cache, data, seq, from+1, group_idx, cur_run);
            }
            // treat as '.'
            //// Not tracking anything, just skip '.'s
            if cur_run == 0{
                let p = dfs(cache, data, seq, from+1, group_idx, cur_run);
                ways += p;
            }
            //// See if it matched
            if cur_run == seq[group_idx]{
                // match
                let w = dfs(cache, data, seq, from+1, group_idx+1, 0);
                cache.insert((from, group_idx, cur_run), w);
                ways += w;
            }
            // treat as '#'
            //// if it would be valid
            if seq[group_idx] > cur_run{
                let p = dfs(cache, data, seq, from+1, group_idx, cur_run+1);
                ways += p;
            }
            cache.insert((from, group_idx, cur_run), ways);
            return ways;
        },
        _ => unreachable!()
    }
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

fn times_5(r: &Row)-> Row{
    Row{
        data: (0..5)
        .map(|_| r.data.clone())
        .collect::<Vec<String>>()
        .join("?"),
        seq: r.seq.repeat(5),
    }

}

pub fn p2 (file: &str) -> usize{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<Row>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().ok())
            .filter_map(|parsed_ip| Some(parsed_ip))
            .map(|i| i.unwrap())
            .collect();
        let ans: usize = data.iter().map(|r| times_5(r))
            .map(|r| solve_dp(&r.data, &r.seq)).sum();
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
