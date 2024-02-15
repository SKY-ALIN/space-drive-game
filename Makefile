SHELL := /bin/bash

# BUILD
build_python_:
	maturin build --manifest-path python/Cargo.toml --release --out dist --interpreter 3.8 3.9 3.10 3.11 3.12
build_core:
	cargo build --package space_drive_game_core --release
build_server:
	cargo build --package space_drive_game_server --release
build: build_python_ build_core build_server

# INSTALL
install_python:
	python -m pip install space_drive_game --no-index --no-deps --find-links dist --force-reinstall

# TEST
test_python:
	python -m pytest python/tests
test_core:
	cargo test --package space_drive_game_core
test: test_core test_python 

# DEBUG
debug_python:
	maturin develop --manifest-path python/Cargo.toml
run:
	cargo run --package space_drive_game_server

# LINTING
lint:
	cargo clippy --all-targets --all-features
	cargo fmt --all --check
	flake8
	find . -iname "*.py" -path "./python/*" | xargs pylint
	isort . --check-only
fix:
	cargo fmt --all
	isort .
