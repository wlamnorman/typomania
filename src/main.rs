mod engine;
mod input;
mod results;
mod terminal_ui;
mod text;
mod word_selector;

use crate::{engine::Engine, input::Input};
use clap::Parser;

fn main() {
    let mut engine = Engine::new(Input::parse());
    engine.run();
}
