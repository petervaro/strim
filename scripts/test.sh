#!/usr/bin/env bash

# SEE: https://bertvv.github.io/cheat-sheets/Bash.html
set -euo pipefail;

function run {
    local version="${1-}";

    bash -c "cargo $version test";
    cd dummy && bash -c "cargo $version test";
}

ARGUMENT="${1-}";
case "$ARGUMENT" in
    'ci')
        run;
        ;;
    'local')
        run +1.79.0;
        run +1.80.0;
        ;;
    *)
        printf \
            'Invalid argument, expected `ci` or `local`, but got: `%s`\n' \
            "$ARGUMENT" \
            >&2;
        exit 1;
        ;;
esac;
