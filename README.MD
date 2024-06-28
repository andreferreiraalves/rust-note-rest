# CLI For migration
cargo install sqlx-cli

# create a migration
sqlx migrate add -r create_notes_table

# perform migration up
sqlx migrate run

# (Bonus!, perform migration down/revert)
sqlx migrate revert