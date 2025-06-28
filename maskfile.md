# Tasks for the project

## build

> Builds all crates and packages

~~~sh
cargo build --release --workspace
uv sync --all-packages
~~~

## fmt

> Formats the code

~~~sh
cargo fmt --all
uv run ruff format
uv run isort .
~~~

## lint

> Lints the code

~~~sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
uv run ruff check
uv run isort --check-only --diff .
uv run mypy --strict --pretty .
~~~

## test

> Runs all tests

<!-- We are waiting on this issue before relying on `nextest` for doc-tests.
https://github.com/nextest-rs/nextest/issues/16
-->

~~~sh
echo "Running Rust tests"
cargo nextest run --all --all-targets --all-features --release --no-tests warn

echo "Running Rust doc-tests"
cargo test --doc --all --all-features --release

echo "Running Python tests"
uv sync --all-packages > /dev/null 2>&1
for dir in $(find . -name pyproject.toml -not -path "./pyproject.toml" -exec dirname {} \;); do
    echo "Running tests in $dir"
    cd "$dir" && uv run pytest ./**/*.py
done
~~~
