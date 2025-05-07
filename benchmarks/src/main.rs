pub(crate) mod cli;

use cli::{Cli, Post};
use std::io::Write;

use clap::Parser;
use post2_elastic_wall::{DefaultHashStrategy, ElasticHashTable};

fn main() -> anyhow::Result<()> {
    match Cli::parse().post {
        Post::Post1 { slots, output } => {
            println!("Using Post 1 (Greedy Hash Table)...");
            println!("Total slots: {}", slots);
            println!("Writing probe data to: {}", output);
            let mut file = std::fs::File::create(&output)?;
            writeln!(file, "load_factor,probes")?;
            run_post1(slots, &mut file)?;
            println!("✅ Done. Wrote output to {output}");
        }

        Post::Post2 {
            slots,
            output,
            balanced,
        } => {
            println!("Using Post 2 (Elastic Hash Table)...");
            println!("Total slots: {}", slots);
            println!("Balanced mode: {}", balanced);
            println!("Writing probe data to: {}", output);
            let mut file = std::fs::File::create(&output)?;
            writeln!(file, "load_factor,probes")?;
            run_post2(slots, balanced, &mut file)?;
            println!("✅ Done. Wrote output to {output}");
        }
    }
    Ok(())
}

fn run_post1(slots: usize, file: &mut std::fs::File) -> anyhow::Result<()> {
    let mut table = post1_invisible_wall::HashTable::<u32, &str>::new(slots);

    for i in 0..slots {
        let probes = table.insert_greedy(i as u32, "val");
        let load_factor = (i + 1) as f64 / slots as f64;
        writeln!(file, "{:.5},{:.5}", load_factor, probes)?;
    }

    Ok(())
}

fn run_post2(slots: usize, balanced: bool, file: &mut std::fs::File) -> anyhow::Result<()> {
    let num_subarrays = 100;
    let slots_per_subarray = slots / num_subarrays;
    let hasher = DefaultHashStrategy;

    let mut table =
        ElasticHashTable::<u32, &str, _>::new(num_subarrays, slots_per_subarray, balanced, hasher);

    for i in 0..(num_subarrays * slots_per_subarray) {
        let probes = table.insert(i as u32, "val"); // assumes `insert()` returns probe count
        let load_factor = (i + 1) as f64 / (num_subarrays * slots_per_subarray) as f64;
        writeln!(file, "{:.5},{:.5}", load_factor, probes)?;
    }

    Ok(())
}
