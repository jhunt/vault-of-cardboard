use std::fmt::Display;
use vault_of_cardboard::scryfall;

fn or<T: Display>(pri: &Option<T>, sec: &str) -> String {
    match pri {
        Some(pri) => format!("{}", pri),
        None => sec.to_string(),
    }
}

fn yn(maybe: &Option<bool>) -> String {
    match maybe {
        Some(true) => "yes",
        Some(false) => "no",
        None => "unknown",
    }.to_string()
}

fn main() {
    let set = scryfall::Set::from_stdin().unwrap();

    println!(
        "----[ {} {}: {} ]-------------------",
        set.object, set.code, set.name
    );
    println!("id:    {}", or(&set.id, "UNKNOWN"));
    println!("type:  {}", or(&set.set_type, "UNKNOWN"));
    println!("date:  {}", set.released_at);
    println!("size:  {}", set.card_count);
    println!(
        "block: {} ({})",
        or(&set.block, "???"),
        or(&set.block_code, "???")
    );
    println!("");
    println!("digital?:   {}", yn(&Some(set.digital)));
    println!("foil only?: {}", yn(&Some(set.foil_only)));
    println!("");
    println!("mtgo code:    {}", or(&set.mtgo_code, "(none)"));
    println!("tcgplayer id: {}", or(&set.tcgplayer_id, "(none)"));
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
        println!(
            "  | costs {} (cmc {})",
            or(&card.mana_cost, "nothing"),
            card.cmc
        );
        println!("  | oracle: {}", or(&card.oracle_text, "(none)"));
        println!("  | flavor: {}", or(&card.flavor_text, "(none)"));
        println!("  |");
        println!("  | colors: {:?}", card.colors);
        println!("  | color identity: {:?}", card.color_identity);
        println!("  | card back {}", or(&card.card_back_id, "UNKNOWN"));
        println!("  |");
        println!("  | {} rarity", card.rarity);
        println!("  | #{}  {}", card.collector_number, card.released_at);
        println!(
            "  | art by {} [{}]",
            card.artist,
            or(&card.illustration_id, "-")
        );
        println!("  | {} frame with {} border", card.frame, card.border_color);
        println!("  | {} layout", card.layout);
        println!("  |");
        println!("  | playable in {:?}", card.games);
        println!("  | legal in {}", card.legalities.as_str());
        println!(
            "  | from {} {} set ({})",
            card.set_name,
            or(&card.set_type, "{unknown}"),
            card.set
        );
        println!("  |      {}", card.set_uri);
        println!("  |");
        if let Some(foil) = card.foil {
            if foil {
                println!("  | this is a FOIL card")
            }
        }
        if let Some(nonfoil) = card.nonfoil {
            if nonfoil {
                println!("  | this is a non-foil card")
            }
        }
        if let Some(full_art) = card.full_art {
            if full_art {
                println!("  | this is a FULL ART card")
            }
        }
        if let Some(oversized) = card.oversized {
            if oversized {
                println!("  | this is an OVERSIZED card")
            }
        }
        if let Some(textless) = card.textless {
            if textless {
                println!("  | this is a TEXTLESS card")
            }
        }

        if let Some(reprint) = card.reprint {
            if reprint {
                println!("  | this is a REPRINT (oracle {})", card.oracle_id)
            }
        }
        if let Some(variation) = card.variation {
            if variation {
                println!("  | this is an ALTERNATE ART card")
            }
        }
        if let Some(reserved) = card.reserved {
            if reserved {
                println!("  | this card is on the RESERVED LIST :(")
            }
        }
        if let Some(story_spotlight) = card.story_spotlight {
            if story_spotlight {
                println!("  | this is a STORY SPOTLIGHT card")
            }
        }
        if let Some(promo) = card.promo {
            if promo {
                println!("  | this is a PROMOTIONAL card")
            }
        }
        if let Some(booster) = card.booster {
            if booster {
                println!("  | available in booster packs")
            }
        }
        if let Some(digital) = card.digital {
            if digital {
                println!("  | available in digital formats")
            }
        }
        println!("  |");
        println!("  |  rulings: {}", card.rulings_uri);
        println!("  |");
        if let Some(prices) = card.prices {
        println!("  | current(ish) prices");
        println!("  |   eur: €{}", or(&prices.eur, "-"));
        println!("  |   tix:  {}", or(&prices.tix, "-"));
        println!("  |   usd: ${}", or(&prices.usd, "-"));
        if let Some(usd) = prices.usd_foil {
            println!("  |   usd: {} (foil)", usd);
        }
        println!("  |");
        }
        println!("  | purchase at");
        println!("  |   tcgplayer:   {}", card.purchase_uris.tcgplayer);
        println!("  |   cardhoarder: {}", card.purchase_uris.cardhoarder);
        println!("  |   cardmarket:   {}", card.purchase_uris.cardmarket);
        println!("  |");
        if let Some(uris) = card.image_uris {
            println!("  | available imagery");
            println!("  |   png:         {}", uris.png);
            println!("  |   small:       {}", uris.small);
            println!("  |   art_crop:    {}", uris.art_crop);
            println!("  |   normal:      {}", uris.normal);
            println!("  |   large:       {}", uris.large);
            println!("  |   border_crop: {}", uris.border_crop);
        }
        println!("  |   high res?    {}", yn(&Some(card.highres_image)));
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
        println!("  |   tcgplayer id: {}", or(&card.tcgplayer_id, "~"));
        println!("  |   mtgo id:      {}", or(&card.mtgo_id, "~"));
        println!("  |   mtgo foil id: {}", or(&card.mtgo_foil_id, "~"));
        println!("  |   multiverse:   {:?}", card.multiverse_ids);
        println!("  |");
        println!("");
    }
}
