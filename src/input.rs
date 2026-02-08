use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub(crate) struct Input {
    /// Number of words to generate
    #[arg(short, long, default_value_t = 12)]
    pub(crate) number_of_words: usize,

    /// Optional seed for re-trying the same set of words
    #[arg(short, long)]
    pub(crate) seed: Option<u64>,
}
