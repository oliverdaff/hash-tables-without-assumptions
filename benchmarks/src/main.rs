use std::io::Write;

use post1_invisible_wall::HashTable;

fn main() -> anyhow::Result<()> {
    let slots = 10_000;
    let mut table = HashTable::<u32, &str>::new(slots);

    let mut file = std::fs::File::create("insert_probes.csv")?;
    writeln!(file, "load_factor,probes")?;

    for i in 0..slots {
        let probes = table.insert_greedy(i as u32, "val");
        let load_factor = (i + 1) as f64 / slots as f64;
        writeln!(file, "{:.5},{:.5}", load_factor, probes)?;
    }

    println!("Wrote insert_probes.csv");
    Ok(())
}
