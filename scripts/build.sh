#!/bin/sh

# Im Projekt-Root:
cargo check

# Oder spezifisch:
cargo check -p dina-core
cargo check -p dina-cli
cargo check -p dina-web

# CLI testen:
cargo run -p dina-cli -- list recipes

# Web testen:
cargo run -p dina-web
# → http://localhost:8080 öffnen
