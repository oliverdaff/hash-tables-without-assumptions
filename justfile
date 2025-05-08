
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


# Post 1 — Greedy insertion (baseline)
probes-at-load-post1:
  cargo run --bin benchmarks -- post1 -s 10000 -o insert_probes_post1.csv

# Post 2 — Elastic fallback variants

# 1. Unbalanced + Unrotated (no rotation, no balancing) — worst case
probes-at-load-post2-unbalanced-unrotated:
  cargo run --bin benchmarks -- post2 -s 10000 -o insert_probes_post2_unbalanced_unrotated.csv

# 2. Unbalanced (rotation only) — helps, but still worse than Greedy
probes-at-load-post2-unbalanced:
  cargo run --bin benchmarks -- post2 -s 10000 -r -o insert_probes_post2_unbalanced.csv

# 3. Balanced (rotation + balanced slot selection) — best performer
probes-at-load-post2-balanced:
  cargo run --bin benchmarks -- post2 -s 10000 -b -r -o insert_probes_post2_balanced.csv

plot_probe_time:
  gnuplot benchmarks/gnuplot/plot_probes.gnuplot

