use rand::seq::{index::sample, SliceRandom};
use rand_xoshiro::Xoroshiro128StarStar;

pub(crate) fn select_words(
    lexicon_words: &[&'static str],
    n_words: usize,
    rng: &mut Xoroshiro128StarStar,
) -> Vec<String> {
    let selected_indices = sample(rng, lexicon_words.len(), n_words).into_vec();

    let mut selected_words: Vec<String> = selected_indices
        .into_iter()
        .map(|idx| lexicon_words[idx].to_string())
        .collect();

    selected_words.shuffle(rng);

    selected_words
}
