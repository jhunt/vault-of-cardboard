#[macro_use]
extern crate clap;
use vault_of_cardboard::cli;

fn main() {
    let args = clap_app!(cardboard =>
                     (version: "1.0")
    (author: "James Hunt <bugs@vaultofcardboard.com>")
    (about: "Vault of Cardboard multi-tool")
    (@subcommand api =>
     (about: "The Vault of Cardboard server API daemon"))
    (@subcommand reconciler =>
     (about: "Reconciles a collection by applying outstanding patches to it.")
     (@arg FILE: +required "Path to the collection.json file to reconcile."))
    (@subcommand rescry =>
     (about: "Transforms ingested Scryfall set/card data into usable formats.")
     (@arg raw:    -r --raw    +takes_value +required "Where to find the raw Scryfall set files.")
     (@arg cards:  -c --cards  +takes_value "Where to put the output cards.json file.")
     (@arg prices: -p --prices +takes_value "Where to put the output prices.json file.")
     (@arg lookup: -l --lookup +takes_value "Where to put the output lookup.json file."))
    )
    .get_matches();

    if args.is_present("api") {
        cli::api::run();
    } else if let Some(subargs) = args.subcommand_matches("reconciler") {
        cli::reconciler::run(
            subargs.value_of("FILE").unwrap()
        );
    } else if let Some(subargs) = args.subcommand_matches("rescry") {
        cli::rescry::run(
            subargs.value_of("raw"),
            subargs.value_of("cards"),
            subargs.value_of("prices"),
            subargs.value_of("lookup"),
        );
    }
}
