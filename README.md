# Rust sample Application

## Setting

### Rust Languege

`rustup override set nightly`

`rustup override set stable`

### Rocket set up

### Diesel set up

1 `cargo install diesel_cli`

2 `diesel setup`

3 `diesel migration generate create_[table name]`

4 `diesel migration run`

or

`diesel migration redo`

## Application command

root file is ./Makefile.
