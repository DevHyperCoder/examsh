use serde::{Deserialize, Serialize};

mod multiple_choice;
mod predict_output;
mod write_code;

pub use multiple_choice::MultipleChoiceQuestions;
pub use predict_output::PredictOutput;
pub use write_code::WriteCode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "qtype")]
pub enum Question {
    RawQuestion(Raw),
    PredictOutputQuestion(PredictOutput),
    MultipleChoiceQuestion(MultipleChoiceQuestions),
    WriteCodeQuestion(WriteCode),
}

impl Question {
    pub fn render(&self) -> String {
        match self {
            Question::RawQuestion(r) => r.render(),
            Question::PredictOutputQuestion(pto) => pto.render(),
            Question::MultipleChoiceQuestion(mcq) => mcq.render(),
            Question::WriteCodeQuestion(wc) => wc.render(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Raw {
    latex: String,
}
impl Raw {
    pub fn render(&self) -> String {
        format!(
            "\\question
{}",
            self.latex
        )
    }
}
