use colored::*;
use std::fmt::Display;

type IndexedSlotRef<'a, K, V> = (usize, &'a Option<(K, V)>);

#[derive(Debug)]
pub struct RenderedSlot {
    pub formatted: ColoredString,
}

pub fn render_table<K: Display>(table: &[Option<(K, impl Display)>]) -> Vec<RenderedSlot> {
    group_slots_by_run(table)
        .into_iter()
        .flat_map(render_slot_run)
        .collect()
}

pub fn display_table(slots: &[RenderedSlot], row_width: usize) {
    println!("\nHash Table View:\n");
    for chunk in slots.chunks(row_width) {
        for slot in chunk {
            print!("{:>5}", slot.formatted);
        }
        println!();
    }
    println!();
}

fn group_slots_by_run<K, V>(table: &[Option<(K, V)>]) -> Vec<Vec<IndexedSlotRef<'_, K, V>>> {
    table
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, slot)| {
            match acc.last_mut() {
                Some(run) if run.last().unwrap().1.is_some() == slot.is_some() => {
                    run.push((i, slot));
                }
                _ => {
                    acc.push(vec![(i, slot)]);
                }
            }
            acc
        })
}

fn render_slot_run<K: Display>(run: Vec<(usize, &Option<(K, impl Display)>)>) -> Vec<RenderedSlot> {
    let run_len = run.len();
    run.into_iter()
        .map(|(_, slot)| RenderedSlot {
            formatted: format_slot(slot, run_len),
        })
        .collect()
}

fn format_slot<K: Display>(entry: &Option<(K, impl Display)>, run_len: usize) -> ColoredString {
    let raw = match entry {
        Some((k, _)) => format!("[{}]", k),
        None => "[ ]".to_string(),
    };

    match (entry.is_some(), run_len) {
        (false, _) => raw.normal(),
        (true, 1) => raw.green(),
        (true, 2..=3) => raw.yellow(),
        _ => raw.red().bold(),
    }
}
