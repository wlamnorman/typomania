use clap::Parser;

const OS_LEXICON_PATH: &str = "/usr/share/dict/words";

#[derive(Parser)]
#[clap(version)]
pub(crate) struct Input {
    /// Number of words to generate
    #[arg(short, long, default_value_t = 25)]
    pub(crate) number_of_words: usize,

    /// Optional seed for re-trying the same set of words
    #[arg(short, long)]
    pub(crate) seed: Option<u64>,

    /// File of line-separated words
    #[arg(short, long, default_value_t=OS_LEXICON_PATH.to_string())]
    pub(crate) lexicon_path: String,
}
