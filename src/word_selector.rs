use rand::{
    seq::{index::sample, SliceRandom},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoroshiro128StarStar;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub(crate) struct WordSelector {
    // as long as files of words are not in multiple millions holding the lexicon in memory
    // is best very large files could be handled by loading a random subset of the words to avoid
    // 10s of MBs of data - 250k words take up approx 1.4 MB
    pub(crate) lexicon: Vec<String>,
    rng: Xoroshiro128StarStar,
    n_words: usize,
}

impl WordSelector {
    pub(crate) fn new(lexicon_path: String, n_words: usize, seed: Option<u64>) -> Self {
        let lexicon =
            Self::load_lexicon(lexicon_path.as_str()).expect("Failed to load lexicon file");

        let n_lexicon_words = lexicon.len();
        if n_words > n_lexicon_words {
            panic!(
                "Requested {} words, but the lexicon file only contains {} lines.",
                n_words, n_lexicon_words
            );
        }

        WordSelector {
            lexicon,
            rng: Xoroshiro128StarStar::seed_from_u64(
                seed.unwrap_or_else(|| rand::thread_rng().gen()),
            ),
            n_words,
        }
    }

    fn load_lexicon(lexicon_path: &str) -> io::Result<Vec<String>> {
        let file = File::open(lexicon_path)?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.map(|l| l.to_ascii_lowercase()))
            .collect()
    }

    pub(crate) fn select_words_from_lexicon(&mut self) -> Vec<String> {
        let selected_indices = sample(&mut self.rng, self.lexicon.len(), self.n_words).into_vec();

        let mut selected_words: Vec<String> = selected_indices
            .into_iter()
            .map(|idx| self.lexicon[idx].clone().to_ascii_lowercase())
            .collect();

        selected_words.shuffle(&mut self.rng);

        selected_words
    }
}
