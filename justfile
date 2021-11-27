set shell := ["sh", "-cu"]
set dotenv-load
set positional-arguments

name := "fake65c02.rs"

alias e := edit
alias b := build
alias t := test
alias w := watch

# -> list
@default: list

# Build project
@build:
    cargo build

# Run project tests
@test:
    cargo test

# Edit this justfile
@edit:
    $EDITOR {{justfile()}}

# Watch for changes to project and execute cmd
@watch cmd="cargo test":
    watchexec -r "{{cmd}}"

# List just targets
@list:
    just --list --unsorted --list-heading "$(printf 'Targets for {{name}}::\n\r')"

# Platform info
@info:
    echo {{name}} :: {{os()}} {{arch()}}

