#! /usr/bin/env bash

CWD=$( pwd )
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

all=(
  single-thread
  # race
  move-closure
  shared-state-naive
  shared-state
)

for program in ${all[@]}; do
  echo "# ${program}"
  cd "${DIR}/${program}"
  out=$( cargo build --release 2>&1 )
  if [[ "$?" == "0" ]]; then
    /usr/bin/time -p ./target/release/${program}
    # rm -rf target
  else
    echo "${out}"
  fi
  echo ""
done

cd "${CWD}"
