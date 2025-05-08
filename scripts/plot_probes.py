import os
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Define probe file paths
probe_files = {
    "Greedy": "./probe-data/insert_probes_post1.csv",
    "Unbalanced + Unrotated": "./probe-data/insert_probes_post2_unbalanced_unrotated.csv",
    "Unbalanced + Rotated": "./probe-data/insert_probes_post2_unbalanced.csv",
    "Balanced + Rotated": "./probe-data/insert_probes_post2_balanced.csv",
}

# Load each dataset
probe_data = {name: pd.read_csv(path) for name, path in probe_files.items()}

# Plot stages — compare one variant at a time to Greedy, plus an all-in-one
plot_stages = [
    ("greedy", ["Greedy"]),
    ("vs_unbalanced_unrotated", ["Greedy", "Unbalanced + Unrotated"]),
    ("vs_unbalanced_rotated", ["Greedy", "Unbalanced + Rotated"]),
    ("vs_balanced_rotated", ["Greedy", "Balanced + Rotated"]),
    ("all_variants", list(probe_files.keys())),
]

# Ensure output directory exists
os.makedirs("./probe-plots", exist_ok=True)

# Set plot style
sns.set_style("whitegrid")
sns.set_palette("colorblind")

# Generate plots
for suffix, series_names in plot_stages:
    plt.figure(figsize=(10, 6))

    for name in series_names:
        df = probe_data[name]
        plt.plot(df["load_factor"], df["probes"], marker="o", label=name)

    plt.title("Probe Count by Load Factor", fontsize=16, fontweight="bold")
    plt.xlabel("Load Factor", fontsize=14)
    plt.ylabel("Average Probes", fontsize=14)
    plt.legend()
    plt.tight_layout()
    plt.savefig(f"./probe-plots/probes_{suffix}.png")
    plt.close()
