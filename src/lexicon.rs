pub(crate) struct Lexicon(Vec<&'static str>);

impl Lexicon {
    pub(crate) fn default() -> Self {
        let raw = include_str!("../assets/lexicons/default.txt");
        let words = raw.lines().collect();
        Self(words)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn words(&self) -> &[&'static str] {
        &self.0
    }
}
