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

    pub fn diff(self) -> Vec<cdif::Line> {
        let mut diffs = vec![];
        for (k, mut l) in self.a {
            l.quantity -= match self.b.get(&k) {
                None => 0,
                Some(l) => l.quantity,
            };
            if l.quantity != 0 {
                diffs.push(l);
            }
        }
        diffs
    }
}

fn main() {
    let a = File::open("a.vif").expect("a.vif should exist...");
    let a = BufReader::new(a);
    let b = File::open("b.vif").expect("b.vif should exist...");
    let b = BufReader::new(b);
    let mut diff = Differ::new();

    let mut n = 0;
    for line in a.lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match cdif::parse_line(&line) {
                    Some(l) => diff.track_a(l),
                    None => println!("a.vif:{}: syntax error!", line),
                };
            },
            Err(e) => println!("read failed at a.vif:{}: {}!", n, e),
        }
    }

    let mut n = 0;
    for line in b.lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match cdif::parse_line(&line) {
                    Some(l) => diff.track_b(l),
                    None => println!("b.vif:{}: syntax error!", line),
                };
            },
            Err(e) => println!("read failed at b.vif:{}: {}!", n, e),
        }
    }

    for line in diff.diff() {
        println!("{}", line.as_cdif_string());
    }
}
