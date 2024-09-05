build:
    cargo check
    cargo build
    cargo build --release

build-local:
    just build
    sudo cp target/release/autoi18n /usr/local/bin/autoi18n-local

format:
    just --fmt --unstable .
    stylua .
    mdsf format .
    npx prettier --write --cache --ignore-unknown .
    cargo fmt

lint:
    cargo fmt -- --check --color always
    cargo clippy --all-targets --all-features -- -D warnings

test:
    just lint
    RUST_BACKTRACE=full cargo test --release

test-coverage:
    cargo llvm-cov clean --workspace
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
    cargo llvm-cov --open

changelog:
    npx auto-changelog -u

precommit:
    just changelog
    cargo clean
    cargo dist init --yes
    just format
    just build
    just lint
    just test
    typos --exclude CHANGELOG.md .
