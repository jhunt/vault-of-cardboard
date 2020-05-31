Vault of Cardboard
==================

This is the code that runs <https://vaultofcardboard.com>.

Developer Tasks
===============

This section contains my personal notes on how to do stuff with
this codebase, since I touch it so infrequently and my children
have utterly destroyed my memory banks.

Run the Integration Tests
-------------------------

Testing VCB requires more than just a `cargo test`.  Luckily, if
you have Docker and Docker-compose installed locally, the Makefile
has everything in it you need to set up the required integration.

To run _just_ the unit-tests, and automatically spinning the
supporting infrastructure (Postgres, Redis, etc.):

    make unit-tests

Integration tests, which run against a compiled API server running
locally, require a bit more finesse.  First, we have to spin up
the integration infrastructure:

    make integration-api

(That really ought to run in a separate terminal, via tmux)

Then, you can run the full integration test suite via:

    make integration-tests

If you want to run both unit and integration tests, and already
have the integration-api target running in the background, you can
just use:

    make test

A more fun developer workflow, however, is to use the
`watch-and-test` target to wait for changes to the Rust code on
the filesystem, and automatically re-run tests against the
integration infrastructure:

    make watch-and-test

I usually run this from one half of a split tmux pane, and then
fire up vim in the other half for hacking.


Browse a Development Copy
-------------------------

If you already have the integration API spinning for your tests,
you can set up a local web UI for browsing on loopback via the
`./test/nginx` script.  Execute this from a separate terminal, and
it will bring up a web server on `localhost:3001`, which you can
use to try out the Javascript.


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
