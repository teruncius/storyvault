default:
    just --list

run:
    pnpm run dev

format:
    pnpm run format

lint:
    pnpm run lint

test:
    cd apps/backend && hurl --test tests/*.hurl
