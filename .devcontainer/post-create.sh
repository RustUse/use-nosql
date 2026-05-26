#!/usr/bin/env bash
set -euo pipefail

rustup component add clippy rustfmt
cargo fetch
