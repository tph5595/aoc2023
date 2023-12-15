use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

#[derive(Default, Debug, Clone)]
struct LensBox{
    map: HashMap<String, usize>,
    queue: LinkedList<String>,
    size: usize
}

impl LensBox {
    fn add(&mut self, k: String, v: usize){
        if let Some(old_v) = self.map.get_mut(&k){
            *old_v = v;
        }else{
            self.map.insert(k.clone(), v);
            self.queue.push_back(k);
            self.size += 1;
        }

    }
    fn remove(&mut self, k: String){
        if let Some(_) = self.map.get(&k){
            self.map.remove(&k);
            self.size -= 1;
        }
    }
    fn total(&mut self)-> usize{
        let mut t = 0;
        loop {
            if self.queue.len() == 0{
                return t; 
            }
            let current = self.queue.pop_back().unwrap();
            // Ensure this isn't a ghost entry
            if let Some(v) = self.map.get(&current){
                t += self.size * v;
                self.size -= 1;
                // in case there are unremoved ones in the queue
                self.map.remove(&current);
            }
        }

    }
}

fn hash_me(s: &String) -> usize{
    let mut current = 0;
    for c in s.chars(){
            current += c as u32;
            current = current * 17;
            current = current % 256;

    }
    current as usize
}

fn add_all(boxes: &mut Vec<LensBox>, data: Vec<String>){
    for s in data{
        if s.chars().last().unwrap() == '-'{
            let mut chars = s.chars();
            chars.next_back();
            let k = chars.as_str().to_owned();
            boxes[hash_me(&k)].remove(k);
        }
        else {
           let mut s_iter = s.split("=");
           let k = s_iter.next().unwrap().to_owned();
           let h = hash_me(&k);
           boxes[h].add(k, s_iter.next().unwrap().parse().unwrap());
        }
        // println!("{:?}", boxes);
    }
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

pub fn p2 (file: &str) -> usize {
    if let Ok(lines) = read_lines(file) {
        let data: Vec<String>= lines
            .into_iter()
            .filter_map(|item| item.ok())
            .flat_map(|s| s.clone().split(",").map(|i| i.to_owned()).collect::<Vec<String>>())
            .collect();
        let boxes = & mut vec![LensBox::default(); 256];
        add_all(boxes, data);
        boxes.iter_mut().enumerate().map(|(i, b)| b.total()*(i+1)).sum()
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
