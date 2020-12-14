#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String // may be missing
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        !self.byr.is_empty() &&
        !self.iyr.is_empty() &&
        !self.eyr.is_empty() &&
        !self.hgt.is_empty() &&
        !self.hcl.is_empty() &&
        !self.ecl.is_empty() &&
        !self.pid.is_empty()
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.

    fn is_valid_part2(&self) -> bool {
        if self.is_valid_part1() == false {
            return false;
        }
        // byr
        if self.byr.cmp(&"1920".to_string()) == Ordering::Less ||
            self.byr.cmp(&"2002".to_string()) == Ordering::Greater {
                return false;
        }
        // iyr
        if self.iyr.cmp(&"2010".to_string()) == Ordering::Less ||
            self.iyr.cmp(&"2020".to_string()) == Ordering::Greater {
                return false;
        }
        // eyr
        if self.eyr.cmp(&"2020".to_string()) == Ordering::Less ||
            self.eyr.cmp(&"2030".to_string()) == Ordering::Greater {
                return false;
        }
        // hgt
        if self.hgt.ends_with("cm") {
            let hgt = self.hgt.trim_end_matches("cm");
            if hgt.cmp(&"150".to_string()) == Ordering::Less ||
                hgt.cmp(&"193".to_string()) == Ordering::Greater {
                    return false;
            }
        } else if self.hgt.ends_with("in") {
            let hgt = self.hgt.trim_end_matches("in");
            if hgt.cmp(&"59".to_string()) == Ordering::Less ||
                hgt.cmp(&"76".to_string()) == Ordering::Greater {
                    return false;
            }
        } else {
            return false;
        }
        // hcl
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        if HCL_RE.is_match(&self.hcl) == false {
            return false;
        }
        // ecl
        let ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if ecl.contains(&self.ecl.as_str()) == false {
            return false;
        }
        // pid
        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        if PID_RE.is_match(&self.pid) == false {
            return false;
        }
        return true;
    }

    fn set(&mut self, key: &str, val: &str) -> () {
        if key == "byr" {
            self.byr = val.to_string();
        } else if key == "iyr" {
            self.iyr = val.to_string();
        } else if key == "eyr" {
            self.eyr = val.to_string();
        } else if key == "hgt" {
            self.hgt = val.to_string();
        } else if key == "hcl" {
            self.hcl = val.to_string();
        } else if key == "iyr" {
            self.iyr = val.to_string();
        } else if key == "ecl" {
            self.ecl = val.to_string();
        } else if key == "pid" {
            self.pid = val.to_string();
        } else if key == "cid" {
            self.cid = val.to_string();
        } else {
            panic!("unknown key {} with value {}", key, val);
        }
    }
}

fn main() {
    let vec = read_file("day4-input.txt");
    println!("part1 = {:?}", validate(&vec, Passport::is_valid_part1));
    println!("part2 = {:?}", validate(&vec, Passport::is_valid_part2));
}

fn read_file(filename: &str) -> Vec<Passport> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };

    let re = Regex::new(r"([a-z]+):(\S+)").unwrap();
    let mut vec = Vec::new();
    let mut pp = Passport::default();
    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        if !sl.is_empty() {
            for cap in re.captures_iter(&sl) {
                parse_passport_fields(&cap, &mut pp);
            }
        } else {
            vec.push(pp);
            pp = Passport::default();
        }
    }
    vec.push(pp);
    vec
}

fn parse_passport_fields(cap: &regex::Captures, pp: &mut Passport) -> () {
    let key = &cap[1];
    let val = &cap[2];
    pp.set(key, val);
}

fn validate(vec: &Vec<Passport>, func: fn(&Passport) -> bool) -> i32 {
    let mut valid_cnt: i32 = 0;
    for pp in vec {
        if func(pp) {
            valid_cnt = valid_cnt + 1;
        }
    }
    return valid_cnt;
}
