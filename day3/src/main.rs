use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const TREE:u8 = 0x23;

fn main() {
    let (vec, max_x) = read_file("day3-input.txt");
    println!("part1 = {}", part1(&vec, max_x));
    println!("part2 = {}", part2(&vec, max_x));
}

fn read_file(filename: &str) -> (Vec<String>, usize) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };

    let mut vec = Vec::new();
    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        vec.push(sl);
    }
    let max_x = vec[0].len() - 1;
    (vec, max_x)
}

fn part1(vec: &Vec<String>, max_x:usize) -> i32 {
    count_trees(&vec, max_x, 3, 1)
}

fn part2(vec: &Vec<String>, max_x:usize) -> u32 {
    let mut sum:u32 = count_trees(&vec, max_x, 1, 1) as u32;
    sum = sum * (count_trees(&vec, max_x, 3, 1) as u32);
    sum = sum * (count_trees(&vec, max_x, 5, 1) as u32);
    sum = sum * (count_trees(&vec, max_x, 7, 1) as u32);
    sum = sum * (count_trees(&vec, max_x, 1, 2) as u32);
    sum
}

fn count_trees(vec: &Vec<String>, max_x:usize,
               step_x:usize, step_y:usize) -> i32 {
    let mut x:usize = 0;
    let mut tree_cnt:i32 = 0;
    let max_y = vec.len();
    let mut y:usize = 0;

    while y < max_y {
        let line = &vec[y];
        if line.as_bytes()[x] == TREE {
            tree_cnt = tree_cnt + 1;
        }
        x = next_x(x, max_x, step_x);
        y = y + step_y;
    }
    tree_cnt
}

fn next_x(x:usize, max_x:usize, step_x:usize) -> usize {
    let mut next_x = x + step_x;
    if next_x > max_x {
        next_x = next_x - max_x - 1;
    }
    next_x
}
