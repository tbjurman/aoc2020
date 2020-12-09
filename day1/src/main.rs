use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let vec = read_file("day1-input.txt");
    println!("part1 = {}", part1(&vec));
    println!("part2 = {}", part2(&vec));
}

fn read_file(filename: &str) -> Vec<i32> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };

    let mut numvec = Vec::new();
    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        let value = sl.parse::<i32>().unwrap();
        numvec.push(value);
    }
    numvec
}

fn part1(vec: &Vec<i32>) -> i32 {
    let target = 2020;
    let len = vec.len();
    for i in 0..len-2 {
        for j in i+1..len-2 {
            if vec[i] + vec[j] == target {
                return vec[i] * vec[j];
            }
        }
    }
    panic!("sum {} not found", target);
}

fn part2(vec: &Vec<i32>) -> i32 {
    let target = 2020;
    let len = vec.len();
    for i in 0..len-2 {
        for j in i+1..len-2 {
            for k in i+2..len-2 {
                if vec[i] + vec[j] + vec[k] == target {
                    return vec[i] * vec[j] * vec[k];
                }
            }
        }
    }
    panic!("sum {} not found", target);
}
