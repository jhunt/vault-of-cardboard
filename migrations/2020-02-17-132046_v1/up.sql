-- Initial Vault of Cardboard Schema,
-- for tracking collectors, their collections (with room to expand later),
-- transactions against those collections, and their decks.

CREATE TABLE collectors (
    id           UUID     NOT NULL PRIMARY KEY,
    username     VARCHAR  NOT NULL UNIQUE,
    email        VARCHAR  NOT NULL,
    password     VARCHAR  NOT NULL
);

CREATE TABLE collections (
    id           UUID     NOT NULL PRIMARY KEY,
    collector    UUID     NOT NULL
      REFERENCES collectors (id) ON DELETE CASCADE
);

CREATE TABLE transactions (
    id           UUID     NOT NULL PRIMARY KEY,
    collection   UUID     NOT NULL
      REFERENCES collections (id) ON DELETE CASCADE,
    dated        DATE     NOT NULL,

    gain         TEXT     NOT NULL DEFAULT '',
    loss         TEXT     NOT NULL DEFAULT '',

    metadata     JSONB    NOT NULL DEFAULT '{}'::json,

    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE decks (
    id           UUID     NOT NULL PRIMARY KEY,
    collector    UUID     NOT NULL
      REFERENCES collectors (id) ON DELETE CASCADE,

    title        VARCHAR  NOT NULL,
    description  TEXT     NOT NULL DEFAULT '',

    main         TEXT     NOT NULL DEFAULT '',
    side         TEXT     NOT NULL DEFAULT '',
    maybe        TEXT     NOT NULL DEFAULT '',

    lineage      UUID     NOT NULL,
    ordinal      INTEGER  NOT NULL,

    UNIQUE (lineage, ordinal),

    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
