#!/usr/bin/env bash

# SEE: https://bertvv.github.io/cheat-sheets/Bash.html
set -euo pipefail;

function run {
    local version="${1-}";
    local colour="${2-}";

    local command="cargo $version test --color $colour"

    bash -c "$command";
    cd dummy && bash -c "$command";
}

ARGUMENT="${1-}";
case "$ARGUMENT" in
    'ci')
        run '' always;
        ;;
    'local')
        run +1.79.0 auto;
        run +1.80.0 auto;
        ;;
    *)
        printf \
            'Invalid argument, expected `ci` or `local`, but got: `%s`\n' \
            "$ARGUMENT" \
            >&2;
        exit 1;
        ;;
esac;
