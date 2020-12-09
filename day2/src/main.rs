use std::cmp::max;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

struct PwdLine {
    char: u8,
    low: u8,
    high: u8,
    pwd: Vec<u8>
}

fn main() {
    let vec = read_file("day2-input.txt");
    println!("part1 = {}", part1(&vec));
    println!("part2 = {}", part2(&vec));
}

fn read_file(filename: &str) -> Vec<PwdLine> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };

    let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    let mut vec = Vec::new();
    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        let cap = match re.captures(&sl) {
            Some(m) => m,
            None => panic!("invalid format {}", &sl)
        };
        let pwd_line = PwdLine {
            char: cap[3].as_bytes()[0],
            low: cap[1].parse::<u8>().unwrap(),
            high: cap[2].parse::<u8>().unwrap(),
            pwd: cap[4].as_bytes().to_vec()
        };
        vec.push(pwd_line);
    }
    vec
}

fn part1(vec: &Vec<PwdLine>) -> i32 {
    let mut valid_cnt = 0;
    for line in vec {
        let cnt = cnt_byte_in_vec(&line.char, &line.pwd);
        if cnt >= line.low && cnt <= line.high {
            valid_cnt += 1;
        }
    }
    valid_cnt
}

fn cnt_byte_in_vec(byte: &u8, vec: &Vec<u8>) -> u8 {
    let mut cnt: u8 = 0;
    for c in vec {
        if c == byte {
            cnt += 1;
        }
    }
    cnt
}

fn part2(vec: &Vec<PwdLine>) -> i32 {
    let mut valid_cnt = 0;
    for line in vec {
        let mut hit = 0;
        if line.pwd.len() as u8 >= max(line.low, line.high) {
            if line.pwd[(line.low - 1) as usize] == line.char {
                hit += 1;
            }
            if line.pwd[(line.high - 1) as usize] == line.char {
                hit += 1;
            }
            if hit == 1 {
                valid_cnt += 1;
            }
        }
    }
    valid_cnt
}
