use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

type ContT = (u64, usize);
type BagMapT = HashMap<u64, Vec<ContT>>;

// EXAMPLE RULES
// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.

fn main() {
    let mut hm = BagMapT::new();
    read_file("day7-input.txt", &mut hm);
    println!("part1 = {}", part1(&hm));
    println!("part2 = {}", part2(&hm));
}

fn mk_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn read_file(filename: &str, bm: &mut BagMapT) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };

    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        parse_line(&sl, bm);
    }
}

fn parse_line(line: &str, bm: &mut BagMapT) {
    let r: Vec<&str> = line.split_terminator(" bags contain ").collect();
    let bag_name_hash = mk_hash(&r[0].to_string());
    let mut cont = Vec::new();
    if r[1].cmp(&"no other bags.".to_string()) != Ordering::Equal {
        for i in 1..r.len() {
            let strip = r[i].replace(" bags", "").replace(" bag", "").replace(".", "");
            let strip_iter = strip.split_terminator(", ");
            for j in strip_iter {
                let mut j_copy: String = j.to_string();
                let idx = j_copy.find(" ").unwrap();
                let sec = j_copy.split_off(idx);
                let real_sec = sec.strip_prefix(" ").unwrap();
                let real_sec_hash = mk_hash(&real_sec);
                let j_copy_i = j_copy.parse::<usize>().unwrap();
                cont.push((real_sec_hash, j_copy_i));
            }
        }
    }
    bm.insert(bag_name_hash, cont);
}

fn part1(bm: &BagMapT) -> i32 {
    let mut count: i32 = 0;
    let search_hash = mk_hash(&"shiny gold".to_string());
    for entry in bm {
        if do_search(search_hash, bm, entry, &mut count) {
            count = count + 1;
        }
    }
    count
}

fn do_search(search_hash: u64, bm: &BagMapT, entry: (&u64, &Vec<ContT>),
              count: &mut i32) -> bool {
    for j in entry.1 {
        if j.0 == search_hash {
            return true;
        }
        let next_entry = (&j.0, bm.get(&j.0).unwrap());
        if do_search(search_hash, bm, next_entry, count) {
            return true;
        }
    }
    false
}

fn part2(bm: &BagMapT) -> usize {
    let lookup_hash = mk_hash(&"shiny gold".to_string());
    do_count(lookup_hash, bm)
}

fn do_count(lookup_hash: u64, bm: &BagMapT) -> usize {
    let bags = bm.get(&lookup_hash).unwrap();
    let mut count: usize = 0;
    for (bag_hash, cnt) in bags {
        count = count + cnt + cnt * do_count(*bag_hash, bm);
    }
    count
}
