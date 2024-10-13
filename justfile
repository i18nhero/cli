build:
    cargo check
    cargo build
    cargo build --release

build-local:
    just build
    sudo cp target/release/i18nhero /usr/local/bin/i18nhero-local

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
    cargo clippy --all-targets --all-features -- -Dclippy::unused_async -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
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

generate-web-api:
    wget https://web.api.i18nhero.com/docs-json -O web.api.i18nhero.spec.json
    cargo progenitor -i web.api.i18nhero.spec.json -o web-api -n web-api -v 0.0.0
    mkdir -p packages/i18nhero/src/codegen/web_api
    rm -rf packages/i18nhero/src/codegen/web_api
    mv web-api/src/lib.rs web-api/src/mod.rs
    mv web-api/src packages/i18nhero/src/codegen/web_api
    rm -rf web-api
    just format

generate-cli-api:
    wget https://cli.api.i18nhero.com/spec -O cli.api.i18nhero.spec.json
    cargo progenitor -i cli.api.i18nhero.spec.json -o cli-api -n cli-api -v 0.0.0
    mkdir -p packages/i18nhero/src/codegen/cli_api
    rm -rf packages/i18nhero/src/codegen/cli_api
    mv cli-api/src/lib.rs cli-api/src/mod.rs
    mv cli-api/src packages/i18nhero/src/codegen/cli_api
    rm -rf cli-api
    just format

precommit:
    # will fail once progenitor adds support for multiple response types
    ! just generate-cli-api
    just generate-web-api
    just changelog
    cargo clean
    cargo dist init --yes
    just format
    just build
    just lint
    just test
    typos --exclude CHANGELOG.md .
