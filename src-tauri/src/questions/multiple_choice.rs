use crate::questions::Question;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct MultipleChoiceQuestions {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_id: usize,
}

impl MultipleChoiceQuestions {
    pub fn render(&self) -> String {
        let choices = self
            .answers
            .iter()
            .enumerate()
            .map(|(id, ans)| {
                let ch = if self.correct_id == id {
                    "\\correctchoice "
                } else {
                    "\\choice "
                };

                format!("{} {}", ch, ans)
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "
\\question
{}
\\begin{{choices}}
{}
\\end{{choices}}
        ",
            self.question, choices
        )
    }
}
