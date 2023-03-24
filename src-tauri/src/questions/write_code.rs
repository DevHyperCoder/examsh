use crate::{questions::Question, utils::wrap_in_code_blocks};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WriteCode {
    question: String,
    output: String,
}

impl Question for WriteCode {
    fn render(&self) -> String {
        format!(
            "
\\question
{}

The output should exactly match what is given below:
{}",
            self.question,
            wrap_in_code_blocks(&self.output)
        )
    }
}
