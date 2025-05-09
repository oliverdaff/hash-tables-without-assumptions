use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub post: Post,
}

#[derive(Subcommand, Debug)]
pub enum Post {
    /// Run the greedy probe demo from Post 1
    Post1 {
        #[arg(long, short, default_value_t = 10000)]
        slots: usize,

        #[arg(long, short, default_value = "probes_post1.csv")]
        output: String,
    },

    /// Run the elastic hashing demo from Post 2
    Post2 {
        /// Total number of slots across all subarrays
        #[arg(long, short, default_value_t = 10000)]
        slots: usize,

        /// Output file for recording probe counts
        #[arg(long, short, default_value = "probes_post2.csv")]
        output: String,

        /// Enable coordinated fallback for insert balancing
        #[arg(short = 'b', long, default_value_t = false)]
        balanced: bool,

        /// Rotate subarray starting point per key hash
        #[arg(short = 'r', long, default_value_t = false)]
        rotate_subarrays: bool,
    },
}
