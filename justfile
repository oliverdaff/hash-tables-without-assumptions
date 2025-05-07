
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


# 1. Post 1 (Greedy insertion)
probes_at_load_post1:
  cargo run --bin benchmarks -- post1 -s 10000 -o insert_probes_post1.csv

# 2. Post 2 (Elastic, unbalanced fallback)
probes_at_load_post2_unbalanced:
  cargo run --bin benchmarks -- post2 -s 10000 -o insert_probes_post2_unbalanced.csv

# 3. Post 2 (Elastic, balanced fallback)
probes_at_load_post2_balanced:
  cargo run --bin benchmarks -- post2 -s 10000 --balanced -o insert_probes_post2_balanced.csv

plot_probe_time:
  gnuplot benchmarks/gnuplot/plot_probes.gnuplot

