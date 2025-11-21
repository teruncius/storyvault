default:
    just --list

run:
    cargo run

test:
    hurl --test tests/*.hurl
