_list:
    @just --list

# Preview the site locally.
preview:
    pnpm start

# Tests examples.
test:
    cd examples && cargo test

# Format project.
fmt:
    oxfmt
