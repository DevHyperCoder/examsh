use crate::utils::wrap_in_code_blocks;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WriteCode {
    question: String,
    output: String,
}

impl WriteCode {
    pub fn render(&self) -> String {
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
