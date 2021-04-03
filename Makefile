TAG      ?= dev
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

release:
	@echo "Checking that VERSION was defined in the calling environment"
	@test -n "$(VERSION)"
	@echo "OK.  VERSION=$(VERSION)"
	DOCKER_BUILDKIT=1 docker build -t $(REGISTER)/api:$(VERSION) -f Dockerfile.api .
	DOCKER_BUILDKIT=1 docker build -t $(REGISTRY)/ux:$(VERSION)  -f Dockerfile.ux .
	docker push $(REGISTER)/api:$(VERSION)
	docker push $(REGISTRY)/ux:$(VERSION)

next:
	@date +v%Y%m%d.%H%M%S

.PHONY: default test rescry-for-tests unit-tests integration-tests watch-and-test schema docker release next
