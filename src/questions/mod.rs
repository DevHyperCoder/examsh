use serde::Deserialize;

mod multiple_choice;
mod predict_output;
mod write_code;

pub use multiple_choice::MultipleChoiceQuestions;
pub use predict_output::PredictOutput;
pub use write_code::WriteCode;

pub trait Question {
    fn render(&self) -> String;
}

#[derive(Deserialize)]
pub struct Raw {
    latex: String,
}
impl Question for Raw {
    fn render(&self) -> String {
        format!(
            "\\question
{}",
            self.latex
        )
    }
}
