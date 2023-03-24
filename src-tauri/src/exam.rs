use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use chrono::Utc;
use serde::Deserialize;

use crate::{
    questions::{MultipleChoiceQuestions, PredictOutput, Question, Raw, WriteCode},
    utils::render_latex,
};

const QPFILE: &str = "/tmp/exam.tex";
const MSFILE: &str = "/tmp/marking.tex";
const OUT_DIR: &str = "/tmp";

#[derive(Deserialize)]
struct ExamSchema {
    course_name: String,
    test_name: String,
    instructions: String,
    marking_instructions: String,
    date_fmt: Option<String>,
}

#[derive(Deserialize)]
enum QuestionType {
    MultipleChoiceQuestion,
    PredictOutput,
    WriteCode,
    Raw,
}

#[derive(Deserialize)]
struct QuestionSchema {
    qtype: QuestionType,
    question: serde_json::Value,
}

pub struct Exam {
    exam_schema: ExamSchema,

    questions: Vec<Box<dyn Question>>,
}

impl Exam {
    pub fn from_exam_file(fname: &str) -> Exam {
        let mut f = File::open(fname).expect("Unable to open exam file");
        let mut content = String::new();
        f.read_to_string(&mut content)
            .expect("Unable to read exam file");
        let exam_schema: ExamSchema =
            serde_json::from_str(&content).expect("Unable to parse exam file");

        // TODO This code makes an assumptions that the questions for this exam are present in a
        // subfolder called "questions".
        // - Only 1 level deep questions are supported and that too for PredictOutput questions.
        // - recursive questions are not supported.

        let mut questions_path = PathBuf::from(fname);
        questions_path.pop(); // pop file name
        questions_path.push("questions");

        let mut questions = vec![];
        for dir in fs::read_dir(&questions_path).expect("Unable to read questions directory") {
            let a = dir.expect("Unable to get dir entry");
            if !a.path().is_file() {
                continue;
            }
            if a.path().extension().expect("Cant get extension") != "json" {
                continue;
            }

            let mut qf = File::open(a.path()).expect("Unable to open question file");
            let mut qcontent = String::new();
            qf.read_to_string(&mut qcontent)
                .expect("Unable to read question file");

            let question_schema: QuestionSchema =
                serde_json::from_str(&qcontent).expect("Unable to parse question file.");
            let question: Box<dyn Question> = match question_schema.qtype {
                QuestionType::PredictOutput => {
                    let mut predict =
                        serde_json::from_value::<PredictOutput>(question_schema.question)
                            .expect("Unable to parse PredictOutput Question");

                    let codes = predict
                        .code_files
                        .iter()
                        .map(|code_fname| {
                            let mut asdf = questions_path.clone();
                            asdf.push(code_fname);
                            let mut f = File::open(asdf).expect("Unable to open code file.");
                            let mut fc = String::new();
                            f.read_to_string(&mut fc).expect("Unable to read file");
                            (code_fname.to_string(), fc)
                        })
                        .collect::<Vec<(String, String)>>();

                    predict.code = codes;
                    Box::new(predict)
                }

                QuestionType::MultipleChoiceQuestion => Box::new(
                    serde_json::from_value::<MultipleChoiceQuestions>(question_schema.question)
                        .expect("Unable to parse MultipleChoiceQuestion Question"),
                ),
                QuestionType::WriteCode => {
                    eprintln!("Code test harness is not yet finalised.");
                    Box::new(
                        serde_json::from_value::<WriteCode>(question_schema.question)
                            .expect("Unable to parse WriteCode Question"),
                    )
                }
                QuestionType::Raw => Box::new(
                    serde_json::from_value::<Raw>(question_schema.question)
                        .expect("Unable to parse Raw Question"),
                ),
            };
            questions.push(question);
        }

        Exam {
            exam_schema,
            questions,
        }
    }

    pub fn make_exam(&self) {
        let today = Utc::now()
            .format(
                &(self
                    .exam_schema
                    .date_fmt
                    .clone()
                    .unwrap_or_else(|| "%d %B %Y".to_string())),
            )
            .to_string();

        let d = include_str!("./base_doc.tex")
            .replace("COURSE", &self.exam_schema.course_name)
            .replace("TEST_NAME", &self.exam_schema.test_name)
            .replace(
                "MARKINGINSTRUCTIONS",
                &self.exam_schema.marking_instructions,
            )
            .replace("INSTRUCTIONS", &self.exam_schema.instructions)
            .replace("DATE", &today)
            .replace("QUESTIONS", self.generate_questions().as_str());

        let exam = d.replace("MODE", "12pt, addpoints");
        let marking = d.replace("MODE", "12pt, answers");

        render_latex(QPFILE, OUT_DIR, &exam);
        render_latex(MSFILE, OUT_DIR, &marking);
    }

    fn generate_questions(&self) -> String {
        self.questions
            .iter()
            .map(|q| q.render())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
