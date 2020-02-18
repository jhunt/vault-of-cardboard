table! {
    collections (id) {
        id -> Uuid,
        collector -> Uuid,
    }
}

table! {
    collectors (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    decks (id) {
        id -> Uuid,
        collector -> Uuid,
        title -> Varchar,
        description -> Text,
        main -> Text,
        side -> Text,
        maybe -> Text,
        lineage -> Uuid,
        ordinal -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        collection -> Uuid,
        dated -> Date,
        gain -> Text,
        loss -> Text,
        created_at -> Timestamptz,
    }
}

joinable!(collections -> collectors (collector));
joinable!(decks -> collectors (collector));
joinable!(transactions -> collections (collection));

allow_tables_to_appear_in_same_query!(
    collections,
    collectors,
    decks,
    transactions,
);
