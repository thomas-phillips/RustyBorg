#!/bin/bash

PASSPHRASE="password"

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
  local file_size="1K"
  local target_dir=$(mktemp -d)

  for i in {1..3}; do
    local file_name=$(random_file_name)
    head -c "$file_size" </dev/urandom >"${target_dir}/${file_name}"

    local file_size=$(increment_size $size)
  done

  echo "$target_dir"
}

create_archive() {
  for ((i = 1; i <= $1; i++)); do
    local files=$(create_files)
    cargo run -- create "$3" -p "$4" --paths "$files" >/dev/null
    sleep $2
  done
}

if [[ "$#" -eq 0 ]]; then
  echo "No arguments supplied"
  exit 1
fi

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

  -c | --create)
    TARGET=$(mktemp -d)
    cargo run -- init "$TARGET" "$PASSPHRASE"
    echo "$TARGET"

    FILES=$(create_files)

    cargo run -- create "$TARGET" -p "$PASSPHRASE" \
      --paths "$FILES" --archive "test"
    # --include-patterns \
    # --exclude-patterns

    rm -rf "$TARGET"
    rm -rf "$FILES"
    ;;

  -l | --list)
    TARGET=$(mktemp -d)
    cargo run -- init "$TARGET" "$PASSPHRASE"
    echo "$TARGET"

    create_archive 5 1 "$TARGET" "$PASSPHRASE"

    cargo run -- list "$TARGET" "$PASSPHRASE"

    rm -rf "$TARGET"
    rm -rf "$FILES"
    ;;

  -v | --verify)
    docker compose up -d
    USER=$(cat docker-compose.yaml | grep USER_NAME | xargs | sed "s/- //g" | awk 'BEGIN { FS = "=" }; {print $2 }')
    CONTAINER_IP=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' openssh-server)
    PORT=$(cat docker-compose.yaml | grep ports --after-context 1 | xargs | sed "s/^[^0-9]*//" | sed "s/:.*//")
    echo "$USER, $CONTAINER_IP, $PORT"

    if ! ssh -o BatchMode=yes -o ConnectTimeout=6 ${USER}@${CONTAINER_IP} -p $PORT exit; then
      echo "SSH Server OFFLINE"
      exit 1
    fi
    echo "SSH Server ONLINE"

    cargo run -- verify $USER $CONTAINER_IP -p $PORT
    ;;

  -s | --schedule)
    SCHEDULE_FILES=$(create_files)
    cargo run -- schedule -e "0/5 * * * * *" -r `mktemp -d` \
      -p "$PASSPHRASE" --paths "$SCHEDULE_FILES"

    ;;

  *)
    echo "Error unknown flag: $1" >&2
    ;;

  esac
  shift

done
