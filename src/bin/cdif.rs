use std::io;
use std::io::prelude::*;

use vault_of_cardboard::parser::{Parser,Collector};

fn main() {
    let mut p = Parser::new();
    let mut n = 0;
    let mut c = Collector::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match p.parse_line(&line) {
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
