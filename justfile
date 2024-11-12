build:
    cargo check
    cargo build
    cargo build --release

build-local:
    just build
    sudo cp target/release/i18nhero /usr/local/bin/i18nhero-local

dist:
    dist init --yes

format:
    just --fmt --unstable .
    stylua .
    mdsf format .
    npx prettier --write --cache --ignore-unknown .
    cargo fmt
    just dist

lint:
    cargo fmt -- --check --color always
    cargo clippy --all-targets --all-features -- -D warnings -Dclippy::perf

lint-aggressive:
    cargo clean
    cargo clippy --fix --allow-staged --all-targets --all-features -- -Dclippy::missing_const_for_fn -Dclippy::unused_async -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions -Aclippy::used_underscore_items -Aclippy::used_underscore_binding
    git restore packages/i18nhero/src/codegen
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

update-help:
    cargo run -p i18nhero-codegen

generate-api-bindings:
    rm -rf public_api
    npx @openapitools/openapi-generator-cli generate -g rust -o public_api -i https://api.i18nhero.com/docs-json --global-property=modelDocs=false,apiDocs=false
    find ./public_api -type f -exec sed -i 's/crate::/crate::codegen::public_api::/g' {} \;
    mkdir -p packages/i18nhero/src/codegen/public_api
    rm -rf packages/i18nhero/src/codegen/public_api
    mv public_api/src/lib.rs public_api/src/mod.rs
    mv public_api/src packages/i18nhero/src/codegen/public_api
    rm -rf public_api
    cargo fmt

precommit:
    just generate-api-bindings
    just changelog
    cargo clean
    just dist
    just build
    just update-help
    just format
    just lint
    just test
    typos --exclude CHANGELOG.md .

cross-target-install TARGET:
    rustup target add {{ TARGET }}

cross-target-build TARGET:
    cross build --target {{ TARGET }}

cross-target-lint TARGET:
    cross clippy --target {{ TARGET }}

cross-target-test TARGET:
    cross test --target {{ TARGET }}

cross-target-validate TARGET:
    just cross-target-install {{ TARGET }}
    just cross-target-build {{ TARGET }}
    just cross-target-lint {{ TARGET }}
    just cross-target-test {{ TARGET }}

cross-linux:
    just cross-target-validate x86_64-unknown-linux-gnu
    just cross-target-validate x86_64-unknown-linux-musl

    just cross-target-validate aarch64-unknown-linux-gnu
    just cross-target-validate aarch64-unknown-linux-musl

cross-windows:
    # Cross does not provide a Dockerfile for Windows
    # just cross-target-validate x86_64-pc-windows-msvc

cross-macos:
    # Cross does not provide a Dockerfile for MacOS
    # just cross-target-validate aarch64-apple-darwin
    # just cross-target-validate x86_64-apple-darwin

# the purpose of this is to check whether packages can be built without external dependencies
cross-all:
    just cross-linux
    just cross-windows
    just cross-macos
