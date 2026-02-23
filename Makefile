
dev: build-lib install
	cd site && npm run dev

clean-site:
	cd site && rm -rf node_modules/
	cd site && rm -rf dist/

clean-lib:
	cd lib && cargo clean
	cd lib && rm -rf pkg/

clean: clean-site clean-lib

install:
	cd site && npm install

build-site: install
	cd site && npm run build

build-lib:
	cd lib && wasm-pack build --release --target web

build: build-lib build-site

deploy: build
	rm -rf /var/www/bitscalc
	cp -r site/dist /var/www/bitscalc

test-lib:
	cd lib && cargo test

test: test-lib

cli:
	cd lib && cargo run --example=cli
