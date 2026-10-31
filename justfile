_list:
    @just --list

# Tests examples.
test:
    cd examples && cargo test

# Format project.
fmt:
    oxfmt
