use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "Hash Table Benchmark")]
#[command(about = "Insert probes for Post 1 and Post 2 hash tables")]
pub struct Cli {
    /// Which post version to benchmark (post-1 or post-2)
    #[arg(short, long, value_enum, default_value_t = Post::Post1)]
    pub post: Post,

    /// Number of slots to use in the table
    #[arg(short, long, default_value_t = 10_000)]
    pub slots: usize,

    /// Output CSV file
    #[arg(short, long, default_value = "insert_probes.csv")]
    pub output: String,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Post {
    Post1,
    Post2,
}
