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

docker:
	docker build -t cardboard:$(TAG) .

push:
	docker build -t $(REGISTRY)/cardboard:$(TAG) .
	docker push $(REGISTRY)/cardboard:$(TAG)

.PHONY: default test unit-tests integration-tests watch-and-test
