SHELL := /bin/bash

build:
	maturin build --release --out dist --interpreter 3.8 3.9 3.10 3.11 3.12
install:
	python -m pip install space_drive_game --no-index --no-deps --find-links dist --force-reinstall
lint:
	cargo clippy
	cargo fmt --all --check
fix:
	cargo fmt --all
