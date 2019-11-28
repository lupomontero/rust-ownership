#! /usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

all=(
  single-thread
  race
  mutex
  mutex-better
)

cd "${DIR}"

for program in ${all[@]}; do
  echo "# ${program}"
  gcc -pthread "${program}.c" -o "${program}"
  /usr/bin/time -p ./${program}
  rm "${program}"
  echo ""
done

cd - > /dev/null
