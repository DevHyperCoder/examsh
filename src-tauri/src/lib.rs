pub mod exam;
pub mod questions;
pub mod utils;

use crate::exam::Exam;

pub fn run() {
    let ex = Exam::from_exam_file("/tmp/asdf/exam.json");
    ex.make_exam();
}
