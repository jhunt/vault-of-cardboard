build:
	sass ui.scss > vcb.css

install: build
	npx minify vcb.css > ../htdocs/css/vcb.css
	rm -rf ../htdocs/fonts/keyrune*
	cp -a keyrune/fonts/* ../htdocs/fonts
	rm -rf ../htdocs/fonts/mana*
	cp -a mana/fonts/* ../htdocs/fonts

refresh:
	rm -rf keyrune/ && mkdir keyrune/
	curl -L https://github.com/andrewgioia/Keyrune/archive/master.tar.gz | tar -xz
	rm -rf keyrune/ && mv Keyrune-master/ keyrune
	
	rm -rf mana/ && mkdir mana/
	curl -L https://github.com/andrewgioia/Mana/archive/master.tar.gz | tar -xz
	rm -rf mana/ && mv Mana-master/ mana

.PHONY: build refresh
