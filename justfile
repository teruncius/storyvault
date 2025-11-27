default:
    just --list

run:
    pnpm run dev

build:
    pnpm run build

format:
    pnpm run format

lint:
    pnpm run lint

test:
    hurl --test apps/backend/tests/*.hurl
