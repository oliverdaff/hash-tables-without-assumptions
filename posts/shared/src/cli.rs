use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum HashStrategyArg {
    /// Rust's real-world DefaultHasher (SipHash 1-3)
    Default,

    /// Modulo-based bad hashing (forces clustering for demos)
    Mod10,
}
