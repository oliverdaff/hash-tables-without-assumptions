use clap::{Parser, ValueEnum};

use post1_invisible_wall::visualizer::{display_table, render_table};
use post1_invisible_wall::{HashStrategy, HashTable};

#[derive(Parser, Debug)]
#[command(name = "HashTable Demo")]
#[command(about = "Visualise open-addressed hash table clustering", long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 30)]
    slots: usize,

    #[arg(short, long, default_value_t = 15)]
    keys: u32,

    /// Hashing strategy to use (default: real-world DefaultHasher)
    #[arg(long, value_enum, default_value_t = HashStrategyArg::Default)]
    hash_strategy: HashStrategyArg,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum HashStrategyArg {
    /// Rust's real-world DefaultHasher (SipHash 1-3)
    Default,

    /// Modulo-based bad hashing (forces clustering for demos)
    Mod10,
}

fn main() {
    let cli = Cli::parse();

    println!("HashTable Config:");
    println!("  - Slots: {}", cli.slots);
    println!("  - Keys: 0..{}", cli.keys - 1);
    println!("  - Hash strategy: {:?}", cli.hash_strategy);

    let mut table = HashTable::<u32, &str>::new(cli.slots);

    let strategy = match cli.hash_strategy {
        HashStrategyArg::Default => HashStrategy::Default,
        HashStrategyArg::Mod10 => HashStrategy::Modulo(10),
    };
    table.set_hash_strategy(strategy);

    for key in 0..cli.keys {
        table.insert_greedy(key, "val");
    }

    let rendered = render_table(&table.table);
    display_table(&rendered, 10);
}
