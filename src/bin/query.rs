extern crate iron;

use vault_of_cardboard::scryfall;

use iron::prelude::*;
use iron::status;

fn main() {
    let set = scryfall::data::Set::from_file("data/cache/mir.set").unwrap();

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
        println!(
            "  | from {} {} set ({})",
            card.set_name, card.set_type, card.set
        );
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
        println!("  |   tcgplayer:   {}", card.purchase_uris.tcgplayer);
        println!("  |   cardhoarder: {}", card.purchase_uris.cardhoarder);
        println!("  |   cardmarket:   {}", card.purchase_uris.cardmarket);
        println!("  |");
        println!("  | available imagery");
        println!("  |   png:         {}", card.image_uris.png);
        println!("  |   small:       {}", card.image_uris.small);
        println!("  |   art_crop:    {}", card.image_uris.art_crop);
        println!("  |   normal:      {}", card.image_uris.normal);
        println!("  |   large:       {}", card.image_uris.large);
        println!("  |   border_crop: {}", card.image_uris.border_crop);
        println!(
            "  |   high res?    {}",
            match card.highres_image {
                true => "available!",
                false => "not available :(",
            }
        );
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
