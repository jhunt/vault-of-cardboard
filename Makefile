VERSION ?= 0.0.1
TAG     ?= dev

default: test

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

docker: rescry-docker api-docker
base-docker:
	docker build -t vault-of-cardboard-base:$(TAG) -f Dockerfile.base .
rescry-docker: base-docker
	docker build -t vault-of-cardboard-rescry:$(TAG) -f Dockerfile.rescry .
api-docker:
	docker build -t vault-of-cardboard-api:$(TAG) -f Dockerfile.api .


.PHONY: default test unit-tests integration-tests watch-and-test
