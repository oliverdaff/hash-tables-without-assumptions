use clap::Parser;

use post2_elastic_wall::{DefaultHashStrategy, ElasticHashTable, HashStrategy, ModuloHashStrategy};
use shared::cli::HashStrategyArg;
use shared::visualizer::{display_table, render_table};

#[derive(Debug, Parser)]
#[command(name = "Elastic Hashing Demo")]
#[command(about = "Visualise elastic subarray-based hash table clustering", long_about = None)]
pub struct Cli {
    /// Number of subarrays
    #[arg(short = 'u', long, default_value_t = 4)]
    pub subarrays: usize,

    /// Number of slots per subarray
    #[arg(short = 's', long, default_value_t = 32)]
    pub slots: usize,

    /// Number of keys to insert
    #[arg(short = 'k', long, default_value_t = 100)]
    pub keys: u32,

    /// Use coordinated fallback to balance insertions across subarrays
    #[arg(short = 'b', long, default_value_t = false)]
    pub balanced: bool,

    /// Whether to rotate subarray starting point based on hash
    #[arg(long, default_value_t = false)]
    pub rotate_subarrays: bool,

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

    let strategy = match cli.hash_strategy {
        HashStrategyArg::Default => HashStrategy::Default(DefaultHashStrategy),
        HashStrategyArg::Mod10 => HashStrategy::Modulo(ModuloHashStrategy(10)),
    };

    let mut table = ElasticHashTable::<u32, &str, _>::new(
        cli.subarrays,
        cli.slots,
        cli.rotate_subarrays,
        cli.balanced,
        strategy,
    );

    for key in 0..cli.keys {
        table.insert(key, "val");
    }

    let rendered = render_table(table.slots());

    display_table(&rendered, cli.slots);
}
