Vault of Cardboard
==================

This is the code that runs <https://vaultofcardboard.com>.

Developer Tasks
===============

This section contains my personal notes on how to do stuff with
this codebase, since I touch it so infrequently and my children
have utterly destroyed my memory banks.

Ingest a Set Locally
--------------------

To ingest a set locally, with all the pagination logic, use the
Docker image, and bind-mount a local directory.

    docker run -it \
      -v $PWD/out:/cache \
      -e CACHE=/cache \
      docker.zyxl.xyz/vcb/ingester:latest \
      IKO [...]

This will paginate, re-assemble, and dump the files in
`/out/dat/`, one file per code (and you can specify multiple
set codes).

Here's an example session from ingesting Ikoria: Lair of
Behemoths:

    INGESTING set IKO...
     - fetching set metadata from Scryfall API...
     - fetching (token) set metadata from Scryfall API...
      - fetching card results page 1 from Scryfall API...
      - fetching card results page 2 from Scryfall API...
      - fetching card results page 3 from Scryfall API...
      - fetching card results page 4 from Scryfall API...

and afterwards:

    $ tree -l out
    out
    ├── dat
    │   └── IKO.json
    ├── scry
    │   └── IKO
    │       ├── cards1.json
    │       ├── cards2.json
    │       ├── cards3.json
    │       ├── cards4.json
    │       ├── set.json
    │       └── tok.json
    └── work
        └── IKO
            ├── cards.json
            ├── next.json
            └── set.json

The `out/dat/IKO.json` contains our combined data.
