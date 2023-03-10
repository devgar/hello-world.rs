use std::io::{Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub fn ask<T: ToString>(items: &[T]) -> Result<usize> {

    ctrlc::set_handler(move || std::process::exit(0))
        .expect("Error setting Ctrl-C handler");
    let pos = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What is Done?")
        .default(0)
        .items(items)
        .interact()?;

    Ok(pos + 1)
}