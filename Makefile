.PHONY: uninstall install
all: uninstall install

install:
	cargo install --path .

uninstall:
	cargo uninstall
