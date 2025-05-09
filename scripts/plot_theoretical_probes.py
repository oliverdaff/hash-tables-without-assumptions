import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

# Ensure output directory exists
os.makedirs("probe-plots", exist_ok=True)

# Set seaborn style
sns.set_style("whitegrid")
sns.set_palette("colorblind")

# Load factors (α), avoiding α = 1
load_factors = np.array([0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99])
delta = 1 - load_factors

# Compute theoretical probe counts
greedy_probes = 1 / delta
elastic_probes = np.log(1 / delta)

# --- Plot 1: Greedy vs Elastic ---
df = pd.DataFrame(
    {
        "Load Factor": np.concatenate([load_factors, load_factors]),
        "Probes": np.concatenate([greedy_probes, elastic_probes]),
        "Strategy": ["Greedy (1 / (1 - α))"] * len(load_factors)
        + ["Elastic (log bound)"] * len(load_factors),
    }
)

plt.figure(figsize=(10, 6))
sns.lineplot(data=df, x="Load Factor", y="Probes", hue="Strategy", marker="o")
plt.title("Theoretical Curves: Greedy vs Elastic", fontsize=16, fontweight="bold")
plt.xlabel("Load Factor", fontsize=14)
plt.ylabel("Expected Probes", fontsize=14)
plt.legend(title="Strategy")
plt.tight_layout()
plt.savefig("probe-plots/probes_theoretical_greedy_vs_elastic.png")
plt.close()

# --- Plot 2: Greedy Only ---
df_greedy = pd.DataFrame(
    {"Load Factor": load_factors, "Expected Probes": greedy_probes}
)

plt.figure(figsize=(10, 6))
sns.lineplot(
    data=df_greedy,
    x="Load Factor",
    y="Expected Probes",
    marker="o",
    label="1 / (1 - α)",
)
plt.title("Greedy’s Theoretical Curve", fontsize=16, fontweight="bold")
plt.xlabel("Load Factor", fontsize=14)
plt.ylabel("Expected Probes", fontsize=14)
plt.legend()
plt.tight_layout()
plt.savefig("probe-plots/probes_theoretical_greedy_only.png")
plt.close()
