default:
    just --list

run:
    pnpm run dev

build:
    pnpm run build

build-windows:
    pnpm run build:windows

format:
    pnpm run format

lint:
    pnpm run lint

test:
    hurl --test apps/backend/tests/*.hurl
