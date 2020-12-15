use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct Group {
    nr: i32,
    qs: HashMap<char, i32>
}

impl Group {
    fn new() -> Self {
        Group {nr: 0, qs: HashMap::new()}
    }
}

fn main() {
    let vec = read_file("day6-input.txt");
    println!("part1 = {}", part1(&vec));
    println!("part2 = {}", part2(&vec));
}

fn read_file(filename: &str) -> Vec<Group> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };

    let mut vec = Vec::new();
    let mut grp = Group::new();

    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        if sl.is_empty() {
            vec.push(grp);
            grp = Group::new();
        } else {
            grp.nr = grp.nr + 1;
            for ch in sl.chars() {
                let nr = match grp.qs.get(&ch) {
                    Some(oldnr) => oldnr + 1,
                    None => 1
                };
                grp.qs.insert(ch, nr);
            }
        }
    }
    vec.push(grp);
    vec
}

fn part1(vec: &Vec<Group>) -> usize {
    let mut res = 0;
    for v in vec {
        res = res + v.qs.len();
    }
    res
}

fn part2(vec: &Vec<Group>) -> usize {
    let mut res = 0;
    for v in vec {
        for (_, nr) in v.qs.iter() {
            if nr == &v.nr {
                res = res + 1;
            }
        }
    }
    res
}
