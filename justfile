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

lint-aggressive:
    cargo clean
    cargo clippy --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
    cargo clean

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
