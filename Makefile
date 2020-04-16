VERSION ?= 0.0.1
TAG     ?= dev

REGISTRY ?= docker.zyxl.xyz/vcb

default: test
build:
	cargo build

htdocs/changelog.json: changelog.yml
	spruce json $< >$@

test: unit-tests integration-tests

unit-tests:
	source test/unit/envrc \
	&& ./test/unit/setup \
	&& cargo test --lib

integration-api:
	source test/integration/envrc \
	&& ./test/integration/setup \
	&& cargo run --bin cardboard api
integration-tests:
	source test/integration/envrc \
	&& ./test/integration/setup \
	&& prove test/integration/*.t

watch-and-test:
	source test/unit/envrc \
	&& ./test/unit/setup \
	&& cargo watch -x 'test --lib'

schema:
	./util/schema > src/schema.rs

docker:
	docker build -t cardboard:$(TAG) .
	docker build -t ingester:$(TAG)   -f docker/ingester/Dockerfile   docker/ingester
	docker build -t proxycache:$(TAG) -f docker/proxycache/Dockerfile docker/proxycache
	
	rm -rf docker/perimeter/htdocs
	cp -a htdocs/ docker/perimeter/htdocs
	docker build -t perimeter:$(TAG)  -f docker/perimeter/Dockerfile  docker/perimeter

tag: docker
	docker tag cardboard:$(TAG)  $(REGISTRY)/cardboard:$(TAG)
	docker tag ingester:$(TAG)   $(REGISTRY)/ingester:$(TAG)
	docker tag perimeter:$(TAG)  $(REGISTRY)/perimeter:$(TAG)
	docker tag proxycache:$(TAG) $(REGISTRY)/proxycache:$(TAG)

push: tag
	docker push $(REGISTRY)/cardboard:$(TAG)
	docker push $(REGISTRY)/ingester:$(TAG)
	docker push $(REGISTRY)/perimeter:$(TAG)
	docker push $(REGISTRY)/proxycache:$(TAG)

compose-up:
	docker-compose -p vcb -f deploy/docker-compose.yml up -d

compose-data:
	docker exec -it vcb_ingest_1 /bin/sh -c 'echo 10E 2ED 3ED 4ED 5DN 5ED 6ED 7ED 8ED 9ED A25 AER AKH ALA ALL APC ARB ARC ARN ATH ATQ AVR BBD BFZ BNG BOK C13 C14 C15 C16 C17 C18 CHK CHR CM2 CMA CMD CN2 CNS CON CSP DD2 DDC DDD DDE DDF DDG DDN DDO DDP DDQ DDR DDS DDT DDU DGM DIS DKA DOM DRK DST DTK E01 E02 ELD EMA EMN EVE EXO EXP FEM FRF FUT GK1 GK2 GNT GPT GRN GTC HML HOU ICE IMA INV ISD JOU JUD KLD KTK LEA LEB LEG LGN LRW M10 M11 M12 M13 M14 M15 M19 M20 MBS MED MH1 MIR MM2 MM3 MMA MMQ MOR MPS MRD NEM NPH ODY OGW ONS ORI PC2 PCA PCY PLC PLS RAV RIX RNA ROE RTR SCG SHM SOI SOK SOM STH THS TMP TOR TSP UDS ULG UMA USG VIS W16 WAR WTH WWK XLN ZEN C19 ELD TBD IKO | sort | xargs -n1 -P16 ingest'
	docker exec -it vcb_api_1 /cardboard rescry -r /cache/dat -c /data/cards.json -p /data/prices.json -l /data/lookup.json

compose-down:
	docker-compose -p vcb -f deploy/docker-compose.yml down

.PHONY: default test unit-tests integration-tests watch-and-test schema docker push compose-up compose-down
