SHELL := /bin/bash

build_python:
	maturin build -m python/Cargo.toml --release --out dist --interpreter 3.8 3.9 3.10 3.11 3.12
install_python:
	python -m pip install space_drive_game --no-index --no-deps --find-links dist --force-reinstall
debug_python:
	maturin develop -m python/Cargo.toml
test_python:
	python -m pytest tests/python/
lint:
	cargo clippy
	cargo fmt --all --check
fix:
	cargo fmt --all
