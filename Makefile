TAG     ?= dev

REGISTRY ?= docker.zyxl.xyz/vcb

default: test
build:
	cargo build

htdocs/changelog.json: changelog.yml
	spruce json $< >$@

test: unit-tests integration-tests

rescry-for-tests:
	cargo run --bin cardboard rescry \
	  --raw test/scryfall/sets/ \
	  --cards test/cards.json \
	  --lookup test/lookup.json  \
	  --prices test/prices.json

unit-tests:
	. test/unit/envrc \
	&& ./test/unit/setup \
	&& cargo test --lib

integration-api:
	. test/integration/envrc \
	&& ./test/integration/setup \
	&& cargo run --bin cardboard api
integration-tests:
	. test/integration/envrc \
	&& ./test/integration/setup \
	&& prove test/integration/*.t

watch-and-test:
	. test/unit/envrc \
	&& ./test/unit/setup \
	&& cargo watch -x 'test --lib'

schema:
	./util/schema > src/schema.rs

docker:
	DOCKER_BUILDKIT=1 docker build -t api:$(TAG) -f Dockerfile.api .
	DOCKER_BUILDKIT=1 docker build -t ux:$(TAG)  -f Dockerfile.ux .

tag: docker
	docker tag api:$(TAG) $(REGISTRY)/api:$(TAG)
	docker tag ux:$(TAG)  $(REGISTRY)/ux:$(TAG)

push: tag
	docker push $(REGISTRY)/api:$(TAG)
	docker push $(REGISTRY)/ux:$(TAG)

release:
	@echo "Checking that VERSION was defined in the calling environment"
	@test -n "$(VERSION)"
	@echo "OK.  VERSION=$(VERSION)"
	make TAG=$(VERSION) REGISTRY=$(REGISTRY) push

next:
	@date +v%Y%m%d.%H%M%S

backend:
	docker-compose -p vcb build
	docker-compose -p vcb up -d

compose-up:
	docker-compose -p vcb -f deploy/docker-compose.yml up -d

compose-data:
	docker exec -it vcb_ingest_1 /bin/sh -c 'echo 10E 2ED 3ED 4ED 5DN 5ED 6ED 7ED 8ED 9ED A25 AER AKH ALA ALL APC ARB ARC ARN ATH ATQ AVR BBD BFZ BNG BOK C13 C14 C15 C16 C17 C18 CHK CHR CM2 CMA CMD CN2 CNS CON CSP DD2 DDC DDD DDE DDF DDG DDN DDO DDP DDQ DDR DDS DDT DDU DGM DIS DKA DOM DRK DST DTK E01 E02 ELD EMA EMN EVE EXO EXP FEM FRF FUT GK1 GK2 GNT GPT GRN GTC HML HOU ICE IMA INV ISD JOU JUD KLD KTK LEA LEB LEG LGN LRW M10 M11 M12 M13 M14 M15 M19 M20 MBS MED MH1 MIR MM2 MM3 MMA MMQ MOR MPS MRD NEM NPH ODY OGW ONS ORI PC2 PCA PCY PLC PLS RAV RIX RNA ROE RTR SCG SHM SOI SOK SOM STH THS TMP TOR TSP UDS ULG UMA USG VIS W16 WAR WTH WWK XLN ZEN C19 ELD TBD IKO M21 | sort | xargs -n1 -P16 ingest'
	docker exec -it vcb_api_1 /cardboard rescry -r /cache/dat -c /data/cards.json -p /data/prices.json -l /data/lookup.json

compose-down:
	docker-compose -p vcb -f deploy/docker-compose.yml down

.PHONY: default test unit-tests integration-tests watch-and-test schema docker push compose-up compose-down
