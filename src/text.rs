use std::fmt::Display;
use termion::{color, style};

#[derive(Default)]
pub(crate) struct Text {
    pub(crate) raw: String,
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self { raw: text }
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self::from(text.to_string())
    }
}

impl From<char> for Text {
    fn from(text: char) -> Self {
        Self::from(text.to_string())
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl Text {
    pub(crate) fn len(&self) -> u16 {
        self.raw.len() as u16
    }

    pub(crate) fn with_color(mut self, color: impl color::Color) -> Self {
        self.raw = format!(
            "{}{}{}",
            color::Fg(color),
            self.raw,
            color::Fg(color::Reset)
        );
        self
    }

    pub(crate) fn with_underline(mut self) -> Self {
        self.raw = format!("{}{}{}", style::Underline, self.raw, style::Reset);
        self
    }

    pub(crate) fn render(&self) -> &str {
        &self.raw
    }

    pub(crate) fn as_chars(&self) -> Vec<char> {
        let chars: Vec<char> = self.raw.chars().collect();
        chars
    }
}
