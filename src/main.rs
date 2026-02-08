mod engine;
mod input;
mod lexicon;
mod results;
mod terminal_ui;
mod text;
mod word_select;

use crate::{engine::Engine, input::Input};
use clap::Parser;

fn main() {
    let mut engine = Engine::new(Input::parse());
    engine.run();
}
