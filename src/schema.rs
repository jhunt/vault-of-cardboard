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
        code -> Varchar,
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
        summary -> Text,
        notes -> Text,
        disposition -> Varchar,
        gain -> Text,
        loss -> Text,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        paid -> Nullable<Int4>,
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
