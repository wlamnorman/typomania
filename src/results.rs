// You managed to type k characters in y seconds with x% accuracy!
// eCPS: ...
// cWPM
// typos: ...
// fucks not given: ...
// fucks given: ... (backspaces)
//
// eCPS = effective characters per second
// cWPM = cursed words per minute

#[derive(Default)]
pub(crate) struct Results {
    pub(crate) ms_elapsed: u128,
    pub(crate) n_chars_to_type: usize,
    pub(crate) n_chars_typed: u16,
    pub(crate) n_typos: u16,
    pub(crate) n_backspaces: u16,
}

impl Results {
    pub(crate) fn new(n_chars_to_type: usize) -> Self {
        Self {
            ms_elapsed: 0,
            n_chars_to_type,
            n_chars_typed: 0,
            n_typos: 0,
            n_backspaces: 0,
        }
    }
}
