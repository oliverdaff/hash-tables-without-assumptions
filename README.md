# Hash Tables Without Assumptions — Blog Series Code

This repository contains Rust code examples for the blog series  
**“Hash Tables Without Assumptions: Rethinking Open Addressing at Scale”**

→ [Read the series on fdmux.dev](https://fdmux.dev/series/hash-tables-without-assumptions)

---

## 📚 Series Overview

This series explores how modern non-greedy probing techniques — like **Elastic Hashing** and **Funnel Hashing** — challenge the classical $\Theta(\log(1/\delta))$ lower bound on probe complexity in open-addressed hash tables.

We walk through each concept with annotated, runnable Rust code.

**Planned posts:**

1. **The Invisible Wall Around Hash Tables**  
   Why greedy probing slows down — and how to see it in code.  
   ✅ *Code included in this repo*

2. **Beating the Bound — Without Reordering**  
   Elastic Hashing spreads keys across subarrays to reduce clustering.

3. **Inside Elastic Hashing**  
   Final probe logic, batch-friendly balancing, and sub-logarithmic probes.

4. **Funnel Hashing and Worst-Case Sanity**  
   A levelled fallback strategy with bounded worst-case probes.

5. **What Faster Hashing Means for Computing**  
   Why this changes the way we design data-heavy systems.

---

## 📁 Repo Layout

This is a Cargo workspace. Each post has its own crate.

```text
hash-tables-without-assumptions/
├── Cargo.toml                     # Workspace definition
├── flake.nix                      # Reproducible dev environment (Nix)
├── justfile                       # Handy project commands
├── post1-invisible-wall/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── post2_elastic-hashing/        # 🚧 Coming soon
└── benches/                      # Criterion benchmarks (future)
```

---

## 🦀 Running Post 1

To run the code for **Post 1**:

```bash
cargo run --bin post1-invisible-wall
cargo run
```

You’ll see a visualisation of how greedy insertion causes clustering — even when the table is mostly empty.

By default, it will initialise a table with 30 slots and insert 15 keys using a simple greedy strategy.

You can customise the run using CLI flags:

```bash
just post1 -- --slots 50 --keys 20 --hash-strategy default
```

**Usage:**

```text
post1-invisible-wall [OPTIONS]

Options:
  -s, --slots <SLOTS>                  Number of slots in the table [default: 30]
  -k, --keys <KEYS>                    Number of keys to insert [default: 15]
      --hash-strategy <HASH_STRATEGY> Hash strategy to use [default: mod10]
                                      [possible values: default, mod10]
  -h, --help                           Print help
```

This lets you simulate different load factors and see how clustering appears — even when the table is mostly empty — based purely on the insertion strategy.

---

## 🧪 Development with Nix

This repo includes a `flake.nix` to provide a reproducible Rust environment:

```bash
nix develop
```

This gives you:

- `rustc`, `cargo`, `rust-analyzer`
- `cargo-watch` and `gnuplot` (for benchmarking)

---

## 🛠 Common Commands with `just`

Run common dev tasks with [`just`](https://github.com/casey/just):

```bash
just list        # Show available tasks
just post1         # Run Post 1 crate
just check       # Check all workspace crates
```

---

## 🔧 Requirements

- Rust 1.70+ (edition 2021)
- [`just`](https://github.com/casey/just) (optional, for convenience)
- [`nix`](https://nixos.org/) (optional, for reproducible dev env)

---

## 📜 License

MIT

---

## 📣 Follow Along

More posts — and their code — will be published here as the series progresses.

For updates, diagrams, and additional commentary:  
[@oliverdaff on LinkedIn](https://www.linkedin.com/in/oliverdaff)
