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

    pub fn get_question_id(&self) -> String {
        match self {
            Question::RawQuestion(r) => r.get_id(),
            Question::PredictOutputQuestion(pto) => pto.get_id(),
            Question::MultipleChoiceQuestion(mcq) => mcq.get_id(),
            Question::WriteCodeQuestion(wc) => wc.get_id(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Raw {
    question: String,
    question_id: String,
}
impl Raw {
    pub fn get_id(&self) -> String {
        self.question_id.to_owned()
    }

    pub fn render(&self) -> String {
        format!(
            "\\question
{}",
            self.question
        )
    }
}
