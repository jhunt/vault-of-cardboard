use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

use vault_of_cardboard::data::cdif;

pub struct Collector {
    map: HashMap<String, cdif::Line>
}

impl Collector {
    pub fn new() -> Collector {
        Collector{
            map: HashMap::new(),
        }
    }

    pub fn track(&mut self, line: cdif::Line) {
        match self.map.get_mut(&line.id()) {
            Some(l) => { l.quantity += line.quantity; }
            None => { self.map.insert(line.id(), line); }
        };
    }

    pub fn print(&self) {
        for (_, l) in &self.map {
            println!("{}", l.as_cdif_string());
        }
    }
}

fn main() {
    let mut n = 0;
    let mut c = Collector::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match cdif::parse_line(&line) {
                    Some(l) => {
                        c.track(l);
                    },
                    None => println!("{}: syntax error!", line),
                };
            },
            Err(e) => println!("read failed at line {}: {}!", n, e),
        }
    }
    c.print();
}
