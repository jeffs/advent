#!/usr/bin/env -S zsh -euo pipefail

for f in src/bin/day*.rs; do
    bin=$(basename $f | cut -d. -f1)
    echo $bin: $(=cargo run --quiet --bin $bin)
done
