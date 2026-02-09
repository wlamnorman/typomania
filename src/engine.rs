use crate::{
    input::Input,
    lexicon::Lexicon,
    results::Results,
    terminal_ui::{TerminalUI, QUIT_KEY, RESTART_KEY},
    word_select::select_words,
};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoroshiro128StarStar;
use std::{io::stdin, time::Instant};
use termion::{event::Key, input::TermRead};

pub(crate) struct GameState {
    input_chars: Vec<char>,
    target_chars: Vec<char>,
    results: Results,
}

impl GameState {
    pub(crate) fn new(target_chars: Vec<char>) -> Self {
        let n_chars_to_type = target_chars.len();
        Self {
            input_chars: Vec::new(),
            target_chars,
            results: Results::new(n_chars_to_type),
        }
    }

    pub(crate) fn is_over(&self) -> bool {
        self.input_chars.len() >= self.target_chars.len()
    }
}

pub(crate) struct Engine {
    rng: Xoroshiro128StarStar,
    lexicon: Lexicon,
    words: Vec<String>,
    terminal_ui: TerminalUI,

    state: GameState,
}

impl Engine {
    pub(crate) fn new(input: Input) -> Self {
        let lexicon = Lexicon::default();
        if input.number_of_words > lexicon.len() {
            panic!(
                "Requested {} words, but the lexicon only contains {} lines.",
                input.number_of_words,
                lexicon.len()
            );
        }

        let mut rng = Xoroshiro128StarStar::seed_from_u64(
            input.seed.unwrap_or_else(|| rand::thread_rng().gen()),
        );

        let words = select_words(lexicon.words(), input.number_of_words, &mut rng);
        let terminal_ui = TerminalUI::new(&words);
        let chars_to_type = terminal_ui.text_lines_as_chars();

        Self {
            rng,
            lexicon,
            words,
            terminal_ui,
            state: GameState::new(chars_to_type),
        }
    }

    fn restart(&mut self) {
        let new_words = select_words(self.lexicon.words(), self.words.len(), &mut self.rng);
        self.terminal_ui.reinitialize(&new_words);
        self.words = new_words;
        let new_chars_to_type = self.terminal_ui.text_lines_as_chars();
        self.state = GameState::new(new_chars_to_type);
    }

    fn get_correct_char(&self) -> char {
        self.state.target_chars[self.state.input_chars.len() - 1]
    }

    pub(crate) fn run(&mut self) {
        let stdin = stdin();
        'gameplay: loop {
            let mut started_typing = false;
            let mut start_time: Option<Instant> = None;
            let mut end_time: Option<Instant> = None;

            for key in stdin.lock().keys() {
                match key.unwrap() {
                    QUIT_KEY => break 'gameplay,
                    RESTART_KEY => {
                        self.restart();
                        continue 'gameplay;
                    }

                    Key::Char(c) => {
                        if !started_typing {
                            started_typing = true;
                            start_time = Some(Instant::now());
                        }

                        self.state.input_chars.push(c);
                        self.state.results.n_chars_typed += 1;
                        if self.state.is_over() {
                            end_time = Some(Instant::now());
                            if self.state.input_chars.last() != self.state.target_chars.last() {
                                self.state.results.n_typos += 1
                            }
                            break;
                        }

                        let correct_char = self.get_correct_char();
                        if c == correct_char {
                            self.terminal_ui.handle_correct_input(correct_char);
                        } else {
                            self.state.results.n_typos += 1;
                            self.terminal_ui.handle_incorrect_input(correct_char);
                        }
                    }

                    Key::Backspace => {
                        if !self.state.input_chars.is_empty() {
                            let correct_char = self.get_correct_char();
                            self.state.input_chars.pop();
                            self.state.results.n_backspaces += 1;
                            self.terminal_ui.handle_backspace(correct_char);
                        }
                    }
                    _ => {}
                }

                self.terminal_ui.flush();
            }

            if let (Some(start), Some(end)) = (start_time, end_time) {
                self.state.results.ms_elapsed = (end - start).as_millis();
            }

            let should_restart = self.display_results(
                &stdin,
                self.state.target_chars.clone(),
                self.state.input_chars.clone(),
            );
            if should_restart {
                self.restart();
                continue;
            }
            break;
        }

        self.terminal_ui.reset_terminal_on_quit()
    }

    fn display_results(
        &mut self,
        stdin: &std::io::Stdin,
        chars_to_type: Vec<char>,
        chars_user_input: Vec<char>,
    ) -> bool {
        self.terminal_ui
            .display_results(&self.state.results, chars_to_type, chars_user_input);

        for key in stdin.lock().keys() {
            match key.unwrap() {
                RESTART_KEY => return true,
                QUIT_KEY => return false,
                _ => {
                    continue;
                }
            }
        }

        false
    }
}
