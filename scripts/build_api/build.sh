#!/usr/bin/env bash

API_NAME=${1:-token_api}
OPENAPI_FILE=${2:-kaleidoerc20mb.swagger.json}

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

rm -r gen/"$API_NAME" > /dev/null 2>&1

# generate code
nix run 'nixpkgs#openapi-generator-cli' -- generate -g rust \
  -i ./data/"$OPENAPI_FILE" \
  -o ./gen/"$API_NAME" \
  --package-name "$API_NAME" \
  --openapi-generator-ignore-list git_push.sh \
  --additional-properties=library=reqwest,packageVersion=0.1.0

# format generated code
cd gen/"$API_NAME" && rustup run nightly cargo fmt -- \
  --config-path ../../scripts/rustfmt.toml && cd - || exit 1

# insert logic to add basic auth to header
sd -s "$(< "$SCRIPT_DIR"/match-agent.txt)" "$(< "$SCRIPT_DIR"/append-agent-basic-auth.txt)" \
  gen/"$API_NAME"/src/apis/default_api.rs

# add base64 crate
sd -s "$(< "$SCRIPT_DIR"/match-cargo.txt)" "$(< "$SCRIPT_DIR"/append-cargo-base64.txt)" \
  gen/"$API_NAME"/Cargo.toml
