use crate::{
    input::Input,
    results::Results,
    terminal_ui::{TerminalUI, QUIT_KEY, RESTART_KEY},
    word_selector::WordSelector,
};
use std::{io::stdin, time::Instant};
use termion::{event::Key, input::TermRead};

pub(crate) struct Engine {
    word_selector: WordSelector,
    words: Vec<String>,
    terminal_ui: TerminalUI,
    chars_user_input: Vec<char>,
    chars_to_type: Vec<char>,
    results: Results,
}

impl Engine {
    pub(crate) fn new(input: Input) -> Self {
        let mut word_selector =
            WordSelector::new(input.lexicon_path, input.number_of_words, input.seed);
        let words = word_selector.select_words_from_lexicon();
        let terminal_ui = TerminalUI::new(&words);
        let chars_to_type = terminal_ui.text_lines_as_chars();
        let n_chars_to_type = chars_to_type.len();
        Self {
            word_selector,
            words,
            chars_user_input: Vec::new(),
            chars_to_type,
            terminal_ui,
            results: Results::new(n_chars_to_type),
        }
    }

    fn restart(&mut self) {
        let new_words = self.word_selector.select_words_from_lexicon();
        self.terminal_ui.reinitialize(&new_words);
        self.words = new_words;
        self.chars_user_input = Vec::new();
        self.chars_to_type = self.terminal_ui.text_lines_as_chars();

        let n_chars_to_type = self.chars_to_type.len();
        self.results = Results::new(n_chars_to_type);
    }

    fn test_is_over(&self) -> bool {
        self.chars_user_input.len() >= self.chars_to_type.len()
    }

    fn get_correct_char(&self) -> char {
        self.chars_to_type[self.chars_user_input.len() - 1]
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

                        self.chars_user_input.push(c);
                        self.results.n_chars_typed += 1;
                        if self.test_is_over() {
                            end_time = Some(Instant::now());
                            if self.chars_user_input.last() != self.chars_to_type.last() {
                                self.results.n_typos += 1
                            }
                            break;
                        }

                        let correct_char = self.get_correct_char();
                        if c == correct_char {
                            self.terminal_ui.handle_correct_input(correct_char);
                        } else {
                            self.results.n_typos += 1;
                            self.terminal_ui.handle_incorrect_input(correct_char);
                        }
                    }

                    Key::Backspace => {
                        if !self.chars_user_input.is_empty() {
                            let correct_char = self.get_correct_char();
                            self.chars_user_input.pop();
                            self.results.n_backspaces += 1;
                            self.terminal_ui.handle_backspace(correct_char);
                        }
                    }
                    _ => {}
                }

                self.terminal_ui.flush();
            }

            if let (Some(start), Some(end)) = (start_time, end_time) {
                self.results.ms_elapsed = (end - start).as_millis();
            }

            let should_restart = self.display_results(
                &stdin,
                self.chars_to_type.clone(),
                self.chars_user_input.clone(),
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
            .display_results(&self.results, chars_to_type, chars_user_input);

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
