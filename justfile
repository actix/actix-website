_list:
    @just --list

# Preview the site locally.
preview locale="en":
    pnpm start --locale {{locale}}

# Tests examples.
test:
    cd examples && cargo test

# Format project.
fmt:
    oxfmt

# Update the saved GitHub stars and latest Actix Web release.
update-project-activity:
    node scripts/update-project-activity.mjs
