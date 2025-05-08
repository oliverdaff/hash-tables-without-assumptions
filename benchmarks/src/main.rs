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
            rotate_subarrays,
            balanced,
        } => {
            println!("Using Post 2 (Elastic Hash Table)...");
            println!("Total slots: {}", slots);
            println!("Rotate subarrays: {}", rotate_subarrays);
            println!("Balanced mode: {}", balanced);
            println!("Writing probe data to: {}", output);
            let mut file = std::fs::File::create(&output)?;
            writeln!(file, "load_factor,probes")?;
            run_post2(slots, rotate_subarrays, balanced, &mut file)?;
            println!("✅ Done. Wrote output to {output}");
        }
    }
    Ok(())
}

/// Run Post 1 benchmark: Greedy hash table insertion across load factors.
/// Averages probes per insert over multiple trials for stable measurement.
fn run_post1(slots: usize, file: &mut std::fs::File) -> anyhow::Result<()> {
    let trials_per_point = 20; // Number of runs per load point
    let load_step = 0.01; // Measure every 1% load

    for step in 1..=(1.0 / load_step) as usize {
        let load_factor = step as f64 * load_step;
        let inserts = (slots as f64 * load_factor).round() as usize;

        let mut total_probes = 0;

        for _ in 0..trials_per_point {
            let mut table = post1_invisible_wall::HashTable::<u32, &str>::new(slots);

            for i in 0..inserts {
                total_probes += table.insert_greedy(i as u32, "val");
            }
        }

        let avg_probes = total_probes as f64 / (inserts * trials_per_point) as f64;
        writeln!(file, "{:.5},{:.5}", load_factor, avg_probes)?;
    }

    Ok(())
}

/// Run Post 2 benchmark: measure average probes across load factors
/// for the Elastic Hash Table, using either balanced or unbalanced fallback.
/// Writes CSV output with `load_factor,avg_probes`.
fn run_post2(
    slots: usize,
    rotate_subarrays: bool,
    balanced: bool,
    file: &mut std::fs::File,
) -> anyhow::Result<()> {
    // Split the table into subarrays for Elastic Hashing
    let num_subarrays = 100;
    let slots_per_subarray = slots / num_subarrays;
    let hasher = DefaultHashStrategy;

    // Benchmarking config
    let trials_per_point = 20; // How many runs per load factor (averaging)
    let load_step = 0.01; // Measure every 1% load increase

    // Sweep from 1% load up to 100%
    for step in 1..=(1.0 / load_step) as usize {
        let load_factor = step as f64 * load_step;
        let inserts = (slots as f64 * load_factor).round() as usize;

        let mut total_probes = 0;

        // Run the experiment multiple times to reduce variance
        for _ in 0..trials_per_point {
            // Create a fresh table for each trial
            let mut table = ElasticHashTable::<u32, &str, _>::new(
                num_subarrays,
                slots_per_subarray,
                rotate_subarrays,
                balanced,
                hasher.clone(), // Clone to preserve same hash strategy across trials
            );

            // Insert up to the current load and count probes
            for i in 0..inserts {
                total_probes += table.insert(i as u32, "val"); // Insert dummy key/value
            }
        }

        // Average probes per insert at this load factor
        let avg_probes = total_probes as f64 / (inserts * trials_per_point) as f64;

        // Write a row to the CSV: load_factor,average_probe_count
        writeln!(file, "{:.5},{:.5}", load_factor, avg_probes)?;
    }

    Ok(())
}
