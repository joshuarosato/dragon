.PHONY: build release run ui install uninstall

PREFIX ?= $(HOME)/.local

build:
	cargo build

release:
	cargo build --release

run:
	cargo run -- $(ARGS)

ui:
	./scripts/regenerate-ui.sh

install: release
	install -Dm755 target/release/dragon "$(PREFIX)/bin/dragon"
	install -Dm644 data/io.github.joshuarosato.Dragon.desktop "$(PREFIX)/share/applications/io.github.joshuarosato.Dragon.desktop"

uninstall:
	rm -f "$(PREFIX)/bin/dragon"
	rm -f "$(PREFIX)/share/applications/io.github.joshuarosato.Dragon.desktop"
