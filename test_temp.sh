#!/bin/bash

PATTERN_FILE="patterns.txt"
PASSPHRASE="password"
SIZE="1K"

create_repo() {
  local target=$(mktemp -d)
  cargo run -- init "$target" "$PASSPHRASE"
  echo "$target"
}

random_file_name() {
  local name=$(cat /dev/urandom | tr -cd "a-f0-9" | head -c 32)
  echo "$name"
}

increment_size() {
  local size="$1"
  local unit="${size: -1}"
  local number="${size%$unit}"

  local new_number=$((number + 1))

  echo "${new_number}${unit}"
}

create_files() {
  local target_dir=$(mktemp -d)
  echo "$target_dir"

  for i in {1..3}; do
    local file_name=$(random_file_name)
    head -c "$SIZE" </dev/urandom >"${target_dir}/${file_name}"

    SIZE=$(increment_size $SIZE)
  done

  echo "$target_dir"
}

while [[ "$#" -gt 0 ]]; do
  case $1 in
  -h | --help)
    cargo run -- help
    ;;

  -t | --test)
    create_files
    ;;

  -i | --init)
    TARGET=$(mktemp -d)
    cargo run -- init "$TARGET" "$PASSPHRASE"
    echo "$TARGET"
    ;;

  -s | --sync)
    TARGET=$(mktemp -d)
    cargo run -- init "$TARGET" "$PASSPHRASE"
    echo "$TARGET"

    FILES=$(create_files)

    cargo run -- create "$TARGET" -p "$PASSPHRASE" \
      --paths "$FILES" --pattern-file "$PATTERN_FILE" \
      --archive "test"

    rm -rf "$FILES"
    ;;

  *)
    echo "Error unknown flag: $1" >&2
    ;;

  esac
  shift

done
