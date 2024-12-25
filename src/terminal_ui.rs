use crate::{results::Results, text::Text};
use std::io::{stdout, Write};
use termion::{
    clear, color,
    cursor::{self},
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

pub(crate) const RESTART_CHAR: char = 'r';
pub(crate) const RESTART_KEY: Key = Key::Ctrl(RESTART_CHAR);

pub(crate) const QUIT_CHAR: char = 'q';
pub(crate) const QUIT_KEY: Key = Key::Ctrl(QUIT_CHAR);

#[derive(Default)]
struct TextLine {
    x: u16,
    y: u16,
    length: u16,
    text: Text,
}

#[derive(Default)]
struct TextCursor {
    curr_text_line: usize,
    curr_char_idx: u16,
    text_lines: Vec<TextLine>,
}

impl TextCursor {
    fn current(&self) -> (u16, u16) {
        let text_line = &self.text_lines[self.curr_text_line];
        (text_line.x + self.curr_char_idx, text_line.y)
    }

    fn next(&mut self) -> (u16, u16) {
        let text_line = &self.text_lines[self.curr_text_line];
        if self.curr_char_idx < text_line.length - 1 {
            self.curr_char_idx += 1;
        } else if self.curr_text_line < self.text_lines.len() {
            self.curr_text_line += 1;
            self.curr_char_idx = 0;
        }
        self.current()
    }

    fn previous(&mut self) -> (u16, u16) {
        let text_line = &self.text_lines[self.curr_text_line];
        if self.curr_char_idx > 0 {
            self.curr_char_idx -= 1;
        } else if self.curr_text_line > 0 {
            self.curr_text_line -= 1;
            self.curr_char_idx = text_line.length;
        }
        self.current()
    }
}

pub(crate) struct TerminalUI {
    stdout: RawTerminal<std::io::Stdout>,
    text_cur_pos: TextCursor,
    term_width: u16,
    term_height: u16,
}

impl TerminalUI {
    pub(crate) fn new(words: &Vec<String>) -> Self {
        let (w, h) = terminal_size().expect("Failed to get terminal size");

        let mut terminal_ui = Self {
            stdout: stdout().into_raw_mode().unwrap(),
            text_cur_pos: TextCursor::default(),
            term_width: w,
            term_height: h,
        };

        terminal_ui.reinitialize(words);

        terminal_ui
    }

    pub(crate) fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub(crate) fn text_lines_as_chars(&self) -> Vec<char> {
        self.text_cur_pos
            .text_lines
            .iter()
            .fold(Vec::<char>::new(), |mut chars, text_line| {
                chars.extend(text_line.text.as_chars());
                chars
            })
    }

    pub(crate) fn reinitialize(&mut self, words: &Vec<String>) {
        self.clear_screen();
        self.text_cur_pos = TextCursor::default();

        let text_lines = self.build_text_lines(&words);
        self.text_cur_pos.text_lines = text_lines;

        self.display_text_to_type();
        self.display_keyboard_shortcuts();

        self.reinitialize_buffer();

        self.flush();
    }

    fn build_text_lines(&self, words: &Vec<String>) -> Vec<TextLine> {
        let text_vec = self.build_vec_of_text(words);
        let center_x = self.term_width / 2;
        let center_y = self.term_height / 2;
        let y_offset = text_vec.len() as u16;

        let mut text_lines = Vec::new();
        for (i, text) in text_vec.into_iter().enumerate() {
            let length = text.len();
            let x_offset = length / 2;
            let x = center_x.saturating_sub(x_offset);
            let y = center_y.saturating_sub(y_offset) + i as u16;
            text_lines.push(TextLine { x, y, length, text });
        }

        text_lines
    }

    fn build_vec_of_text(&self, words: &[String]) -> Vec<Text> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        let max_line_len = (0.2 * self.term_width as f32).floor() as usize;

        for word in words {
            if current_line.len() + word.len() > max_line_len {
                lines.push(Text::from(current_line));
                current_line = String::new();
            }

            current_line.push_str(word);
            current_line.push(' ');
        }

        if !current_line.is_empty() {
            lines.push(Text::from(current_line.trim_end()));
        }

        lines
    }

    pub(crate) fn clear_screen(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
        self.flush();
    }

    fn reinitialize_buffer(&mut self) {
        let text_line = &self.text_cur_pos.text_lines[0];

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(text_line.x, text_line.y),
            cursor::BlinkingBar,
            cursor::Show
        )
        .unwrap();
    }

    fn display_text_to_type(&mut self) {
        for text_line in &self.text_cur_pos.text_lines {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(text_line.x, text_line.y),
                text_line.text.render()
            )
            .unwrap();
        }
        self.flush();
    }

    pub(crate) fn display_keyboard_shortcuts(&mut self) {
        let center_x = self.term_width / 2;
        let start_y = self.term_height;

        let restart_sc = format!("ctrl-{}", RESTART_CHAR);
        let restart_desc = "to restart";
        let x = center_x - (restart_sc.len() + restart_desc.len()) as u16 / 2;
        let y = start_y - 1;
        write!(
            self.stdout,
            "{}{} {}",
            cursor::Goto(x, y),
            Text::from(restart_sc).with_color(color::Green).render(),
            restart_desc
        )
        .unwrap();

        let quit_sc = format!("ctrl-{}", QUIT_CHAR);
        let quit_desc = "to quit";
        let x = center_x - (quit_sc.len() + quit_desc.len()) as u16 / 2;
        let y = start_y;
        write!(
            self.stdout,
            "{}{} {}",
            cursor::Goto(x, y),
            Text::from(quit_sc).with_color(color::Red).render(),
            quit_desc
        )
        .unwrap();

        self.flush();
    }

    fn display_text(&mut self, text: &Text) {
        write!(self.stdout, "{}", text).unwrap();
    }

    fn move_buffer_to_cursor_next(&mut self) {
        let (x, y) = self.text_cur_pos.next();
        write!(self.stdout, "{}", cursor::Goto(x, y)).unwrap();
    }

    fn move_buffer_to_cursor_prev(&mut self) {
        let (x, y) = self.text_cur_pos.previous();
        write!(self.stdout, "{}", cursor::Goto(x, y)).unwrap();
    }

    fn move_buffer_to_cursor(&mut self) {
        let (x, y) = self.text_cur_pos.current();
        write!(self.stdout, "{}", cursor::Goto(x, y)).unwrap();
    }

    pub(crate) fn handle_correct_input(&mut self, correct_char: char) {
        self.display_text(&Text::from(correct_char).with_color(color::Green));
        self.move_buffer_to_cursor_next();
    }

    pub(crate) fn handle_incorrect_input(&mut self, correct_char: char) {
        self.display_text(
            &Text::from(correct_char)
                .with_color(color::Red)
                .with_underline(),
        );
        self.move_buffer_to_cursor_next();
    }

    pub(crate) fn handle_backspace(&mut self, last_char: char) {
        self.move_buffer_to_cursor_prev();
        self.display_text(&Text::from(last_char).with_color(color::Reset));
        self.move_buffer_to_cursor();
    }

    pub(crate) fn display_results(
        &mut self,
        results: &Results,
        chars_to_type: Vec<char>,
        chars_user_input: Vec<char>,
    ) {
        self.clear_screen();
        self.display_keyboard_shortcuts();

        let incorrect_chars_at_end = chars_to_type
            .iter()
            .zip(chars_user_input.iter())
            .filter(|(c1, c2)| c1 != c2)
            .count();

        let center_x = self.term_width / 2;
        let start_y = self.term_height / 3;

        let accuracy = 100.0 * incorrect_chars_at_end as f32 / (results.n_chars_to_type as f32);
        let seconds_elapsed = results.ms_elapsed as f32 / 1000.0;
        let n_fucks_given = results.n_typos as i16 - incorrect_chars_at_end as i16;

        let main_line = format!(
           "You mangaged to type {} characters in {} seconds with {}% errors and {} number of fucks given about typos!",
            results.n_chars_typed, seconds_elapsed, accuracy, n_fucks_given
        );

        let x = center_x - main_line.len() as u16 / 2;
        let y = start_y - 1;
        write!(self.stdout, "{}{}", cursor::Goto(x, y), main_line).unwrap();

        self.flush();
    }

    pub(crate) fn reset_terminal_on_quit(&mut self) {
        let _ = self.stdout.suspend_raw_mode();

        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::SteadyBlock,
            cursor::Goto(1, 1)
        )
        .expect("Could not reset terminal while exiting");

        self.flush();
    }
}
