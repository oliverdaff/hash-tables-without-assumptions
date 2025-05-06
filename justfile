
#  Dev tooling

check:
    cargo check

build:
    cargo build

test:
    cargo test

bench:
    cargo bench -p benchmarks

bench-report:
    xdg-open target/criterion/report/index.html

watch:
    cargo watch -x check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all -- --check

typos:
    typos

udeps:
    cargo +nightly udeps

audit:
    cargo audit

deny:
    cargo deny check

sort:
    cargo sort --check

bloat:
    cargo bloat --release --crates

expand:
    cargo expand

ci:
    just fmt
    just lint
    just typos
    just audit
    just deny

post1  *args:
  cargo run --bin post1-invisible-wall -- {{args}}

post2  *args:
  cargo run --bin post2-elastic-wall -- {{args}}


probes_at_load:
  cargo run --bin benchmarks -- -s 10000 -o results.csv

plot_probe_time:
  gnuplot benchmarks/gnuplot/plot_probes.gnuplot

