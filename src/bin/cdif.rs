#[macro_use]
extern crate clap;

use vault_of_cardboard::prelude::*;
use vault_of_cardboard::data::cdif;

fn main() {
    let app = clap_app!(cdifdiff =>
        (version: "1.0")
        (about: "Parse, inspect, and reformat CDIF data.")
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
        let file = cdif::File::from_stdin().unwrap();
        for (_, l) in &file.lines {
            println!("{}", l.as_cdif_string());
        }
        std::process::exit(0);
    }

    if let Some(sub) = app.subcommand_matches("diff") {
        let a = cdif::File::from_file(sub.value_of("OLD").unwrap()).unwrap();
        let b = cdif::File::from_file(sub.value_of("NEW").unwrap()).unwrap();

        let diff = cdif::File::diff(&a, &b);
        for (_,line) in diff.lines {
            println!("{:+} {}", line.quantity, line.id());
        }
    }
}
