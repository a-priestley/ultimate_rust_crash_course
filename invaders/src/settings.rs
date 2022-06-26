// use crossterm::style::Color;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    rows: usize,
    columns: usize,
    color: String,
}

impl Settings {
    pub fn color(&self) -> &str {
        self.color.as_ref()
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn new() -> Self {
        Self {
            rows: 20,
            columns: 40,
            color: "Blue".to_string(),
        }
    }
}
