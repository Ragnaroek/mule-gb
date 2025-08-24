run-mgb BINARY:
    @cargo run --bin mgb --features mgb -- ../_testdata/{{BINARY}} --format json
