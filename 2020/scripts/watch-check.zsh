#!/usr/bin/env zsh

declare tmp=$(mktemp -d)
trap "rm -rf $tmp" EXIT

declare a=$tmp/a b=$tmp/b

if [ $(uname) = Linux ]; then
    # Print timestamps of regular files under the current directory.
    ts() { fdfind --type=file --exec-batch stat --format=%Y; }
else
    ts() { fd --type=file --exec-batch stat -f %m; }
fi

clear-check() {
    local run='cargo --color=always --quiet check'
    clear
    echo -e "\e[2m[$(date +%T)] $run" "$@" "\e[22m\n"
    ${=run} "$@" |& head -20
    echo -ne "\n\e[2m[$(date +%T)] $?\e[22m"
}

ts >$a
clear-check "$@"

while true; do
    sleep 0.5
    ts >$b
    if ! cmp --silent $a $b; then
        mv $b $a
        clear-check "$@"
    fi
done
