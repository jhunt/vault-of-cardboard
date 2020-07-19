-- Your SQL goes here
CREATE TABLE goals (
    id           UUID  NOT NULL PRIMARY KEY,
    collector    UUID  NOT NULL
      REFERENCES collectors (id) ON DELETE CASCADE,

    name      TEXT     NOT NULL,
    ordinal   INTEGER  NOT NULL,

    target    TEXT     NOT NULL,
    goal      TEXT     NOT NULL DEFAULT 'owned',

    total     INTEGER DEFAULT NULL,
    progress  INTEGER DEFAULT NULL,

    UNIQUE(collector, name),
    UNIQUE(collector, ordinal),

    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
