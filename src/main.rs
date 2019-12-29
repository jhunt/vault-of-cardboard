extern crate iron;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

use iron::prelude::*;
use iron::status;

#[derive(Deserialize)]
struct Legality {
    legacy: String,
    oldschool: String,
    vintage: String,
    pauper: String,
    duel: String,
    standard: String,
    future: String,
    frontier: String,
    modern: String,
    commander: String,
    penny: String,
}

impl Legality {
    fn as_str(&self) -> String {
        fn is_legal(s: &String) -> bool {
            match s.as_ref() {
                "legal" => true,
                _ => false,
            }
        }

        let mut split = "";
        let mut s = String::new();

        if is_legal(&self.legacy) {
            s.push_str(split);
            s.push_str("legacy");
            split = ", ";
        }

        if is_legal(&self.oldschool) {
            s.push_str(split);
            s.push_str("oldschool");
            split = ", ";
        }

        if is_legal(&self.vintage) {
            s.push_str(split);
            s.push_str("vintage");
            split = ", ";
        }

        if is_legal(&self.pauper) {
            s.push_str(split);
            s.push_str("pauper");
            split = ", ";
        }

        if is_legal(&self.duel) {
            s.push_str(split);
            s.push_str("duel");
            split = ", ";
        }

        if is_legal(&self.standard) {
            s.push_str(split);
            s.push_str("standard");
            split = ", ";
        }

        if is_legal(&self.future) {
            s.push_str(split);
            s.push_str("future");
            split = ", ";
        }

        if is_legal(&self.frontier) {
            s.push_str(split);
            s.push_str("frontier");
            split = ", ";
        }

        if is_legal(&self.modern) {
            s.push_str(split);
            s.push_str("modern");
            split = ", ";
        }

        if is_legal(&self.commander) {
            s.push_str(split);
            s.push_str("commander");
            split = ", ";
        }

        if is_legal(&self.penny) {
            s.push_str(split);
            s.push_str("penny");
            split = ", ";
        }

        let _ = split;

        s
    }
}

#[derive(Deserialize)]
struct PurchaseURIs {
    tcgplayer: String,
    cardhoarder: String,
    cardmarket: String,
}

#[derive(Deserialize)]
struct ImageURIs {
    png: String,
    small: String,
    art_crop: String,
    normal: String,
    large: String,
    border_crop: String,
}

#[derive(Deserialize)]
pub struct Prices {
    eur: String,
    usd_foil: Option<String>,
    tix: String,
    usd: String,
}

#[derive(Deserialize)]
pub struct Card {
    object: String,
    id: String,
    oracle_id: String,

    name: String,
    type_line: String,
    oracle_text: String,
    flavor_text: Option<String>,

    artist: String,
    illustration_id: String,

    released_at: String,
    collector_number: String,

    cmc: f32,
    mana_cost: String,

    legalities: Legality,
    purchase_uris: PurchaseURIs,
    set: String,
    set_name: String,
    set_type: String,
    set_uri: String,

    frame: String,
    border_color: String,
    layout: String,

    tcgplayer_id: u32,
    multiverse_ids: Vec<u32>,
    mtgo_foil_id: u32,
    mtgo_id: u32,

    scryfall_uri: String,
    scryfall_set_uri: String,

    lang: String,
    prints_search_uri: String,
    highres_image: bool,

    foil: bool,
    nonfoil: bool,
    full_art: bool,
    oversized: bool,
    textless: bool,
    reprint: bool,
    reserved: bool,
    variation: bool,

    booster: bool,
    digital: bool,

    story_spotlight: bool,
    promo: bool,

    prices: Prices,
    card_back_id: String,
    games: Vec<String>,

    uri: String,
    related_uris: HashMap<String, String>,
    rulings_uri: String,
    image_uris: ImageURIs,

    rarity: String,
    color_identity: Vec<String>,
    colors: Vec<String>,
}

#[derive(Deserialize)]
pub struct Set {
    object: String,
    id: String,

    code: String,
    name: String,
    set_type: String,
    released_at: String,
    card_count: u32,

    block: String,
    block_code: String,

    digital: bool,
    foil_only: bool,

    mtgo_code: String,
    tcgplayer_id: u32,

    icon_svg_uri: String,
    uri: String,
    search_uri: String,

    cards: Vec<Card>,
}

fn main() {
    let mut file = match File::open("data/cache/mir.set") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open mir: {}", why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let s = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read mir: {}", why.description()),
        Ok(_) => s,
    };

    let set: Set = match serde_json::from_str(&s) {
        Err(why) => {
            if why.is_syntax() {
                panic!(
                    "couldn't parse mir: (syntax error on line {}, column {}): {}",
                    why.line(),
                    why.column(),
                    why.description()
                );
            } else if why.is_data() {
                panic!(
                    "couldn't parse mir: (semantic error on line {}, column {}): {}",
                    why.line(),
                    why.column(),
                    why.description()
                );
            } else {
                panic!("couldn't parse mir: {}", why.description());
            }
        }
        Ok(set) => set,
    };

    println!(
        "----[ {} {}: {} ]-------------------",
        set.object, set.code, set.name
    );
    println!("id:    {}", set.id);
    println!("type:  {}", set.set_type);
    println!("date:  {}", set.released_at);
    println!("size:  {}", set.card_count);
    println!("block: {} ({})", set.block, set.block_code);
    println!("");
    println!(
        "digital?:   {}",
        match set.digital {
            true => "yes",
            false => "no",
        }
    );
    println!(
        "foil only?: {}",
        match set.foil_only {
            true => "yes",
            false => "no",
        }
    );
    println!("");
    println!("mtgo code:    {}", set.mtgo_code);
    println!("tcgplayer id: {}", set.tcgplayer_id);
    println!("");
    println!("icon (svg):      {}", set.icon_svg_uri);
    println!("scryfall uri:    {}", set.uri);
    println!("scryfall search: {}", set.search_uri);
    println!("");

    for card in set.cards {
        println!(
            "  |####[ {} {} {} ]####################################",
            card.object, card.id, card.lang
        );
        println!("  |");
        println!("  | {} ({})", card.name, card.type_line);
        println!("  | costs {} (cmc {})", card.mana_cost, card.cmc);
        println!("  | oracle: {}", card.oracle_text);
        println!(
            "  | flavor: {}",
            match card.flavor_text {
                Some(t) => t,
                None => String::from("(none)"),
            }
        );
        println!("  |");
        println!("  | colors: {:?}", card.colors);
        println!("  | color identity: {:?}", card.color_identity);
        println!("  | card back {}", card.card_back_id);
        println!("  |");
        println!("  | {} rarity", card.rarity);
        println!("  | #{}  {}", card.collector_number, card.released_at);
        println!("  | art by {} [{}]", card.artist, card.illustration_id);
        println!("  | {} frame with {} border", card.frame, card.border_color);
        println!("  | {} layout", card.layout);
        println!("  |");
        println!("  | playable in {:?}", card.games);
        println!("  | legal in {}", card.legalities.as_str());
        println!("  | from {} {} set ({})", card.set_name, card.set_type, card.set);
        println!("  |      {}", card.set_uri);
        println!("  |");
        if card.foil {
            println!("  | this is a FOIL card")
        }
        if card.nonfoil {
            println!("  | this is a non-foil card")
        }
        if card.full_art {
            println!("  | this is a FULL ART card")
        }
        if card.oversized {
            println!("  | this is an OVERSIZED card")
        }
        if card.textless {
            println!("  | this is a TEXTLESS card")
        }
        if card.reprint {
            println!("  | this is a REPRINT (oracle {})", card.oracle_id)
        }
        if card.variation {
            println!("  | this is an ALTERNATE ART card")
        }
        if card.reserved {
            println!("  | this card is on the RESERVED LIST :(")
        }
        if card.story_spotlight {
            println!("  | this is a STORY SPOTLIGHT card")
        }
        if card.promo {
            println!("  | this is a PROMOTIONAL card")
        }
        if card.booster {
            println!("  | available in booster packs")
        }
        if card.digital {
            println!("  | available in digital formats")
        }
        println!("  |");
        println!("  |  rulings: {}", card.rulings_uri);
        println!("  |");
        println!("  | current(ish) prices");
        println!("  |   eur: €{}", card.prices.eur);
        println!("  |   tix:  {}", card.prices.tix);
        println!("  |   usd: ${}", card.prices.usd);
        if let Some(usd) = card.prices.usd_foil {
            println!("  |   usd: {} (foil)", usd);
        }
        println!("  |");
        println!("  | purchase at");
        println!("  |   tcgplayer:   {}",   card.purchase_uris.tcgplayer);
        println!("  |   cardhoarder: {}", card.purchase_uris.cardhoarder);
        println!("  |   cardmarket:   {}",  card.purchase_uris.cardmarket);
        println!("  |");
        println!("  | available imagery");
        println!("  |   png:         {}", card.image_uris.png);
        println!("  |   small:       {}", card.image_uris.small);
        println!("  |   art_crop:    {}", card.image_uris.art_crop);
        println!("  |   normal:      {}", card.image_uris.normal);
        println!("  |   large:       {}", card.image_uris.large);
        println!("  |   border_crop: {}", card.image_uris.border_crop);
        println!("  |   high res?    {}", match card.highres_image {
            true => "available!", false => "not available :(" });
        println!("  |");
        println!("  | other web resources:");
        println!("  |    - uri: {}", card.uri);
        println!("  |    - scryfall: {}", card.scryfall_uri);
        println!("  |    - scryfall (set): {}", card.scryfall_set_uri);
        println!("  |    - scryfall (prints): {}", card.prints_search_uri);
        for (name, uri) in card.related_uris {
            println!("  |    - {}: {}", name, uri)
        }
        println!("  |");
        println!("  | known elsewhere as");
        println!("  |   tcgplayer id: {}", card.tcgplayer_id);
        println!("  |   mtgo id:      {}", card.mtgo_id);
        println!("  |   mtgo foil id: {}", card.mtgo_foil_id);
        println!("  |   multiverse:   {:?}", card.multiverse_ids);
        println!("  |");
        println!("");
    }

    Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, "Hello World!"))))
        .http("localhost:3000")
        .unwrap();
}
