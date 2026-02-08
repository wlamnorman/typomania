use crate::lexicon::Lexicon;
use rand::{
    seq::{index::sample, SliceRandom},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoroshiro128StarStar;

pub(crate) struct WordSelector {
    // as long as files of words are not in multiple millions holding the lexicon in memory
    // is best very large files could be handled by loading a random subset of the words to avoid
    // 10s of MBs of data - 250k words take up approx 1.4 MB
    pub(crate) lexicon: Lexicon,
    rng: Xoroshiro128StarStar,
    n_words: usize,
}

impl WordSelector {
    pub(crate) fn new(lexicon: Lexicon, n_words: usize, seed: Option<u64>) -> Self {
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

    pub(crate) fn select_words_from_lexicon(&mut self) -> Vec<String> {
        let selected_indices = sample(&mut self.rng, self.lexicon.len(), self.n_words).into_vec();

        let mut selected_words: Vec<String> = selected_indices
            .into_iter()
            .map(|idx| self.lexicon.words()[idx].to_string())
            .collect();

        selected_words.shuffle(&mut self.rng);

        selected_words
    }
}
