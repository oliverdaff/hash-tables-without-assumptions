use clap::Parser;

use post1_invisible_wall::{HashStrategy, HashTable};
use shared::cli::HashStrategyArg;
use shared::visualizer::{display_table, render_table};

#[derive(Parser, Debug)]
#[command(name = "HashTable Demo")]
#[command(about = "Visualise open-addressed hash table clustering", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 30)]
    pub slots: usize,

    #[arg(short, long, default_value_t = 15)]
    pub keys: u32,

    /// Hashing strategy to use (default: real-world DefaultHasher)
    #[arg(long, value_enum, default_value_t = HashStrategyArg::Default)]
    pub hash_strategy: HashStrategyArg,
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
