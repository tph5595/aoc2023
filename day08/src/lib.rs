use std::collections::HashMap;
use num::integer::lcm;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn p1 (file: &str) -> u128{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<String>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().unwrap())
            .collect();
        let mut iter = data.into_iter();
        let dir = iter.next().unwrap();
        let trans: Vec<Vec<String>>= iter.skip(1)
            .map(|i| 
                 i.replace(|c: char| !(c.is_uppercase() || c.is_whitespace()), "")
                 .split_whitespace()
                 .map(|i| i.to_owned())
                 .collect::<Vec<String>>())
            .collect();
        let map: HashMap<String, (String, String)>= trans.into_iter().fold(HashMap::new(), |mut m, v| {
                                         m.insert(v.get(0).unwrap().to_owned(), 
                                                  (v.get(1).unwrap().to_owned(), 
                                                   v.get(2).unwrap().to_owned()));
                                         m
        });
        let mut state = "AAA";
        let mut count = 0;
        for d in dir.chars().into_iter().cycle(){
            if state == "ZZZ"{
                break;
            }
            if d == 'L' 
            {
                state = &map.get(state).unwrap().0 ;
                count  = count + 1;
            }
            else {
                state = &map.get(state).unwrap().1 ;
                count = count + 1;
            }
        }
        return count;
    }
    else {
        panic!("File not found")
    }
}

pub fn p2(file: &str) -> u128{
    if let Ok(lines) = read_lines(file) {
        let data: Vec<String>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|i| i.parse().unwrap())
            .collect();
        let mut iter = data.into_iter();
        let dir = iter.next().unwrap();
        let trans: Vec<Vec<String>>= iter.skip(1)
            .map(|i| 
                 i.replace(|c: char| !(c.is_uppercase() || c.is_whitespace()), "")
                 .split_whitespace()
                 .map(|i| i.to_owned())
                 .collect::<Vec<String>>())
            .collect();
        let starts: Vec<String> = trans.clone().iter()
            .filter(|i| i.get(0).unwrap().chars().last().unwrap() == 'A')
            .map(|i| i.get(0).unwrap().to_owned())
            .collect();
         let map: HashMap<String, (String, String)> = trans.into_iter()
             .map(|i| (i[0].to_owned(), (i[1].to_owned(), i[2].to_owned())))
             .collect();
        // let map: HashMap<String, (String, String)>= trans.into_iter().fold(HashMap::new(), |mut m, v| {
        //                                  m.insert(v.get(0).unwrap().to_owned(), 
        //                                           (v.get(1).unwrap().to_owned(), 
        //                                            v.get(2).unwrap().to_owned()));
        //                                  m
        // });
        let mut ans: Vec<u128> = Vec::new();
        for i in starts {
            let mut state: & String = &i;
            let mut count = 0;
            for d in dir.chars().into_iter().cycle(){
                if state.chars().last().unwrap() == 'Z'{
                    break;
                }
                if d == 'L' 
                {
                    state = &map.get(state).unwrap().0 ;
                    count  = count + 1;
                }
                else {
                    state = &map.get(state).unwrap().1 ;
                    count = count + 1;
                }
            }
        ans.push(count);
        }
        return ans.into_iter().fold(1, |current_lcm, n| lcm(current_lcm, n));
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
