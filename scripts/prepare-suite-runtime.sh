#!/bin/sh
set -eu

fail() {
  printf 'error: %s\n' "$1" >&2
  exit 1
}

usage() {
  printf '%s\n' \
    'usage: scripts/prepare-suite-runtime.sh [--binary <path>] <destination>' >&2
  exit 2
}

script_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)
binary="$script_root/target/release/repostat"

case "$#" in
  1)
    destination=$1
    ;;
  3)
    [ "$1" = "--binary" ] || usage
    binary=$2
    destination=$3
    ;;
  *)
    usage
    ;;
esac

[ -e "$binary" ] || fail "release binary does not exist: $binary"
[ -f "$binary" ] || fail "release binary is not a regular file: $binary"
[ ! -L "$binary" ] || fail "release binary must not be a symlink: $binary"
[ -x "$binary" ] || fail "release binary is not executable: $binary"
[ ! -e "$destination" ] || fail "destination already exists: $destination"

destination_parent=$(dirname -- "$destination")
destination_name=$(basename -- "$destination")
case "$destination_name" in
  ''|.|..) fail "destination must name a new runtime directory" ;;
esac

mkdir -p -- "$destination_parent"
destination_parent=$(CDPATH= cd -- "$destination_parent" && pwd -P)
destination="$destination_parent/$destination_name"
[ ! -e "$destination" ] || fail "destination already exists: $destination"

manifest="$script_root/ai-mux.extension.json"
schema="$script_root/schemas/repostat-metrics-v1.schema.json"
[ -f "$manifest" ] && [ ! -L "$manifest" ] || fail "extension manifest is invalid"
[ -f "$schema" ] && [ ! -L "$schema" ] || fail "result schema is invalid"

staging=$(mktemp -d "$destination_parent/.${destination_name}.staging.XXXXXX")
cleanup() {
  if [ -n "$staging" ] && [ -d "$staging" ]; then
    rm -rf -- "$staging"
  fi
}
trap cleanup EXIT HUP INT TERM

chmod 0700 "$staging"
mkdir -m 0700 "$staging/schemas"
install -m 0500 "$binary" "$staging/repostat"
install -m 0400 "$manifest" "$staging/ai-mux.extension.json"
install -m 0400 "$schema" "$staging/schemas/repostat-metrics-v1.schema.json"
mv -- "$staging" "$destination"
staging=

printf '%s\n' "$destination"
