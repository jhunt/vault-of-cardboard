JS_FILES := index.js
JS_FILES += api.js
JS_FILES += cdif.js
JS_FILES += draft.js
JS_FILES += helpers.js
JS_FILES += query.js
JS_FILES += vault.js
JS_FILES += when.js

JS_FILES += ui/docs.js
JS_FILES += ui/set-list.js

DIST_FILES := cardboard.js
DIST_FILES += cardboard.min.js
DIST_FILES += cardboard.js.gz
DIST_FILES += cardboard.min.js.gz

default: test $(DIST_FILES)
	ls -lah $(DIST_FILES)

test:
	./this coverage npm test

watch-and-test:
	make test || true
	fswatch $(JS_FILES) test/*.js | xargs -n1 make test

coverage: test
	open coverage/index.html

build: $(DIST_FILES)
clean:
	rm -f $(DIST_FILES)

install: build
	cp cardboard.js ../htdocs/js/cardboard.min.js

cardboard.js: $(JS_FILES)
	./this browserify --standalone cardboard index.js -o $@
cardboard.min.js: cardboard.js
	./this minify $< > $@

%.js.gz: %.js
	gzip -kf $<

.PHONY: test coverage
