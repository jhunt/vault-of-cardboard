#[macro_use]
extern crate clap;
use std::io::{self, BufReader};
use std::fs::File;
use std::process::exit;

use vault_of_cardboard::data::cdif;

fn main() {
    let app = clap_app!(cdifdiff =>
        (version: "1.0")
        (author: "James Hunt <bugs@vaultofcardboard.com>")
        (@subcommand fmt =>
            (about: "Reformat and consolidate a CDIF on standard input to standard output."))
        (@subcommand diff =>
            (about: "Compare to CDIF files and generates a logical CDIF patch.")
            (@arg OLD: +required "The first (base) file to consider.")
            (@arg NEW: +required "The second (changed) file to consider."))
    )
    .get_matches();

    if let Some(_) = app.subcommand_matches("fmt") {
        let file = cdif::File::read(io::stdin().lock());
        for (_, l) in &file.lines {
            println!("{}", l.as_cdif_string());
        }
        exit(0);
    }

    if let Some(sub) = app.subcommand_matches("diff") {
        let a_file = sub.value_of("OLD").unwrap();
        let a = File::open(a_file).expect(&format!("{} should exist...", a_file));
        let a = cdif::File::read(BufReader::new(a));

        let b_file = sub.value_of("NEW").unwrap();
        let b = File::open(b_file).expect(&format!("{} should exist...", b_file));
        let b = cdif::File::read(BufReader::new(b));

        let diff = cdif::File::diff(&a, &b);
        for (_,line) in diff.lines {
            println!("{:+} {}", line.quantity, line.id());
        }
    }
}
