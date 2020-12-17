build:
	#sassc main.scss > vcb.css
	../js/node_modules/.bin/node-sass main.scss > vcb.css

install: build
	mkdir -p ../htdocs/css
	npx minify vcb.css > ../htdocs/css/vcb.css
	
	rm -rf ../htdocs/css/keyrune && mkdir -p ../htdocs/css/keyrune
	cp -a keyrune/fonts ../htdocs/css/keyrune
	
	rm -rf ../htdocs/css/mana && mkdir -p ../htdocs/css/mana
	cp -a mana/fonts ../htdocs/css/mana

refresh:
	rm -rf keyrune/ && mkdir keyrune/
	# https://github.com/andrewgioia/keyrune
	curl -L https://github.com/andrewgioia/keyrune/archive/master.tar.gz | tar -xz
	rm -rf keyrune/ && mv keyrune-master/ keyrune
	
	rm -rf mana/ && mkdir mana/
	# https://github.com/andrewgioia/mana
	curl -L https://github.com/andrewgioia/mana/archive/master.tar.gz | tar -xz
	rm -rf mana/ && mv mana-master/ mana

.PHONY: build refresh
