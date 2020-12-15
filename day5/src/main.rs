use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::Chars;

const ROW: u16 = 7;
const SEAT: u16 = 3;

fn main() {
    let vec = read_file("day5-input.txt");
    let seat_ids = calc_seat_ids(&vec);
    println!("part1 = {}", part1(&seat_ids));
    println!("part2 = {}", part2(&seat_ids));
}

fn read_file(filename: &str) -> Vec<String> {
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
    vec
}

fn calc_seat_ids(vec: &Vec<String>) -> Vec<u16> {
    let mut seat_ids = Vec::new();
    for line in vec {
        let (rowxx, seatxx) = line.split_at(7);
        let row = calc_bsp(rowxx.chars(), ROW);
        let seat = calc_bsp(seatxx.chars(), SEAT);
        let sid = row * 8 + seat;
        seat_ids.push(sid);
    }
    seat_ids
}

fn calc_bsp(chs: Chars, size: u16) -> u16 {
    let mut res: u16 = 0;
    let mut pos: u16 = size;
    for ch in chs {
        pos = pos - 1;
        res = res | char_to_bit(ch, pos);
    }
    res
}

fn char_to_bit(ch: char, pos: u16) -> u16 {
    match ch {
        'F' | 'L' => 0 << pos,
        'B' | 'R' => 1 << pos,
        _ => panic!("invalid format char {}", ch)
    }
}

fn part1(vec: &Vec<u16>) -> u16 {
    let mut max_sid: u16 = 0;
    for sid in vec {
        if *sid > max_sid {
            max_sid = *sid;
        }
    }
    max_sid
}

fn part2(vec: &Vec<u16>) -> u16 {
    let mut cvec = vec.clone();
    let size = cvec.len();
    cvec.sort();

    for i in 1..size-3 {
        if cvec[i+1] - cvec[i] == 2 {
            return cvec[i] + 1;
        }
    }
    panic!("seat not found - overbooked?");
}
