use std::io;
use std::io::prelude::*;
use std::time::Instant;

use vault_of_cardboard::cdif;

fn main() {
    let mut n = 0;
    let stdin = io::stdin();

    let now = Instant::now();
    for line in stdin.lock().lines() {
        n = n + 1;
        match line {
            Ok(line) => {
                match cdif::Line::parse(&line) {
                    Some(_) => (),
                    None => println!("{}: syntax error!", line),
                };
            },
            Err(e) => println!("read failed at line {}: {}!", n, e),
        }
    }
    println!("{}:{}", n, now.elapsed().as_millis());
}
