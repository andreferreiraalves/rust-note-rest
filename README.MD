# CLI For migration
cargo install sqlx-cli

# create a migration
sqlx migrate add -r create_notes_table

# perform migration up
sqlx migrate run

# (Bonus!, perform migration down/revert)
sqlx migrate revert

# run app in watch mode
cargo watch -q -c -w src/ -x run