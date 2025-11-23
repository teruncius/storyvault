default:
    just --list

run:
    pnpm run dev

test:
    cd apps/backend && hurl --test tests/*.hurl

lint:
    pnpm run lint
