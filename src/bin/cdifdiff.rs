#[macro_use]
extern crate clap;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

use vault_of_cardboard::data::cdif;

pub struct Differ {
    a: HashMap<String, cdif::Line>,
    b: HashMap<String, cdif::Line>,
}

impl Differ {
    pub fn new() -> Self {
        Self{
            a: HashMap::new(),
            b: HashMap::new(),
        }
    }

    fn track_in(map: &mut HashMap<String, cdif::Line>, line: cdif::Line) {
        match map.get_mut(&line.id()) {
            Some(l) => { l.quantity += line.quantity; }
            None => { map.insert(line.id(), line); }
        };
    }

    pub fn track_a(&mut self, line: cdif::Line) {
        Self::track_in(&mut self.a, line);
    }
    pub fn track_b(&mut self, line: cdif::Line) {
        Self::track_in(&mut self.b, line);
    }

    pub fn diff(mut self) -> Vec<cdif::Line> {
        let mut diffs = vec![];
        for (k, mut l) in self.a {
            l.quantity = match &self.b.get(&k) {
                None => 0,
                Some(b) => b.quantity - l.quantity,
            };
            if l.quantity != 0 {
                diffs.push(l);
            }
            self.b.remove(&k);
        }
        for (_, l) in self.b {
            diffs.push(l);
        }
        diffs
    }
}

fn main() {
    let matches = clap_app!(cdifdiff =>
        (version: "1.0")
        (author: "James Hunt <bugs@vaultofcardboard.com>")
        (about: "Compare to CDIF files and generates a logical CDIF patch.")
        (@arg OLD: +required "The first (base) file to consider.")
        (@arg NEW: +required "The second (changed) file to consider.")).get_matches();

    let a_file = matches.value_of("OLD").unwrap();
    let a = File::open(a_file).expect(&format!("{} should exist...", a_file));
    let a = BufReader::new(a);

    let b_file = matches.value_of("NEW").unwrap();
    let b = File::open(b_file).expect(&format!("{} should exist...", b_file));
    let b = BufReader::new(b);

    let mut diff = Differ::new();

    let mut n = 0;
    for line in a.lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match cdif::parse_line(&line) {
                    Some(l) => diff.track_a(l),
                    None => println!("{}:{}: syntax error!", a_file, line),
                };
            },
            Err(e) => println!("read failed at {}:{}: {}!", a_file, n, e),
        }
    }

    let mut n = 0;
    for line in b.lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match cdif::parse_line(&line) {
                    Some(l) => diff.track_b(l),
                    None => println!("{}:{}: syntax error!", b_file, line),
                };
            },
            Err(e) => println!("read failed at {}:{}: {}!", b_file, n, e),
        }
    }

    for line in diff.diff() {
        if line.quantity != 0 { // workaround for a bug
            println!("{:+} {}", line.quantity, line.id());
        }
    }
}
