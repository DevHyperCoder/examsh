use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{errors::ExamshError, questions::Question, utils::render_latex};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExamSchema {
    pub course_name: String,
    pub test_name: String,
    pub instructions: String,
    pub marking_instructions: String,
    pub date_fmt: Option<String>,
}

#[derive(Deserialize, Serialize)]
enum QuestionType {
    MultipleChoiceQuestion,
    PredictOutput,
    WriteCode,
    Raw,
}

#[derive(Clone, Debug, Serialize)]
pub struct Exam {
    exam_schema: ExamSchema,
    exam_dir: PathBuf,
    questions: Vec<Question>,
}

impl Exam {
    pub fn get_questions(&self) -> &Vec<Question> {
        &self.questions
    }

    // This method does not expect the question_id to change. If the question id changes, the file
    // should be renamed otherwise there would be a data inconsistency
    pub fn edit_question(&mut self, q: Question) -> Result<&mut Exam, ExamshError> {
        let mut q_path = self.exam_dir.clone();
        q_path.push("questions");
        q_path.push(format!("examsh-{}.json", q.get_question_id()));

        if !q_path.exists() || q_path.is_dir() {
            return Err(ExamshError::Unexpected("Could not find question".into()));
        }

        let question: Question = match q {
            Question::PredictOutputQuestion(mut predict) => {
                let mut newf = self.exam_dir.clone();
                newf.push("questions");
                predict.parse_code(&newf);
                Question::PredictOutputQuestion(predict)
            }

            _ => q,
        };

        match OpenOptions::new().truncate(true).write(true).open(&q_path) {
            Ok(mut f) => match serde_json::to_string_pretty(&question) {
                Ok(c) => match write!(f, "{}", c) {
                    Ok(_) => Ok(self),
                    Err(_) => Err(ExamshError::WriteFile(q_path)),
                },
                Err(_) => Err(ExamshError::Unexpected("Unable to make question".into())),
            },
            Err(_) => Err(ExamshError::OpenFile(q_path)),
        }
    }

    pub fn add_question(&mut self, q: Question) -> Result<&mut Exam, ExamshError> {
        let question: Question = match q {
            Question::PredictOutputQuestion(mut predict) => {
                let mut newf = self.exam_dir.clone();
                newf.push("questions");
                predict.parse_code(&newf);
                Question::PredictOutputQuestion(predict)
            }

            _ => q,
        };

        let mut newf = self.exam_dir.clone();
        newf.push("questions");
        newf.push(format!("examsh-{}.json", question.get_question_id()));

        let mut f = match OpenOptions::new().create(true).write(true).open(&newf) {
            Err(_) => {
                return Err(ExamshError::CreateFile(newf));
            }
            Ok(f) => f,
        };
        let s = match serde_json::to_string_pretty(&question) {
            Err(_) => return Err(ExamshError::Unexpected("Unable to make question".into())),
            Ok(s) => s,
        };

        match write!(f, "{}", s) {
            Ok(_) => {
                self.questions.push(question);
                Ok(self)
            }
            Err(_) => Err(ExamshError::WriteFile(newf)),
        }
    }
    pub fn get_identifier(&self) -> String {
        format!(
            "{}_{}",
            self.exam_schema.course_name, self.exam_schema.test_name
        )
    }

    pub fn edit_exam_schema(
        &mut self,
        new_exam_schema: ExamSchema,
    ) -> Result<&mut Exam, ExamshError> {
        let mut examjson = self.exam_dir.clone();
        examjson.push("exam.json");

        let mut f = match OpenOptions::new().write(true).open(&examjson) {
            Ok(f) => f,
            Err(_) => return Err(ExamshError::OpenFile(examjson)),
        };

        let ex = match serde_json::to_string_pretty(&new_exam_schema) {
            Err(_) => return Err(ExamshError::Unexpected("Unable to make exam schema".into())),
            Ok(s) => s,
        };

        match write!(f, "{}", ex) {
            Ok(_) => {
                self.exam_schema = new_exam_schema;
                Ok(self)
            }
            Err(_) => Err(ExamshError::WriteFile(examjson)),
        }
    }

    pub fn create_exam(exam_schema: ExamSchema, dir: &Path) -> Result<Exam, ExamshError> {
        let mut d = dir.to_path_buf();
        d.push("exam.json");
        match File::create(&d) {
            Err(_) => Err(ExamshError::CreateFile(d)),
            Ok(mut f) => match serde_json::to_string_pretty(&exam_schema) {
                Ok(s) => match write!(f, "{}", s) {
                    Ok(_) => {
                        d.pop();
                        d.push("questions");
                        match fs::create_dir(&d) {
                            Err(_) => Err(ExamshError::Unexpected(format!(
                                "Unable to create questions directory at: {}",
                                d.display()
                            ))),
                            Ok(_) => Ok(Exam {
                                exam_dir: dir.to_path_buf(),
                                questions: vec![],
                                exam_schema,
                            }),
                        }
                    }
                    Err(_) => Err(ExamshError::WriteFile(d)),
                },
                Err(_) => Err(ExamshError::Unexpected(
                    "Unable to make exam file.".to_string(),
                )),
            },
        }
    }
    pub fn from_exam_file(fname: PathBuf) -> Result<Exam, ExamshError> {
        let mut f = match File::open(&fname) {
            Ok(f) => f,
            Err(_) => return Err(ExamshError::OpenFile(fname)),
        };

        let mut content = String::new();
        if f.read_to_string(&mut content).is_err() {
            return Err(ExamshError::ReadFile(fname));
        }

        let exam_schema: ExamSchema = match serde_json::from_str(&content) {
            Err(_) => {
                return Err(ExamshError::ParseExamFile("exam".into(), fname));
            }
            Ok(s) => s,
        };

        // TODO This code makes an assumptions that the questions for this exam are present in a
        // subfolder called "questions".
        // - Only 1 level deep questions are supported and that too for PredictOutput questions.
        // - recursive questions are not supported.

        let mut questions_path = PathBuf::from(&fname);
        questions_path.pop(); // pop file name
        questions_path.push("questions");

        let mut questions = vec![];
        let dir_content = match fs::read_dir(&questions_path) {
            Err(_) => {
                return Err(ExamshError::Unexpected(format!(
                    "Unable to read questions directory: {}",
                    questions_path.display()
                )))
            }
            Ok(e) => e,
        };

        for dir in dir_content {
            let a = match dir {
                Err(_) => {
                    return Err(ExamshError::Unexpected(format!(
                        "Unable to get directory entires of {}",
                        questions_path.display()
                    )))
                }
                Ok(a) => a,
            };
            if !a.path().is_file() {
                continue;
            }
            let path = a.path();
            let e = path.extension();
            let ext = match e {
                Some(ext) => ext,
                None => {
                    return Err(ExamshError::Unexpected(format!(
                        "Can not get file extension for {}",
                        a.path().to_string_lossy()
                    )))
                }
            };
            if ext != "json" {
                continue;
            }

            let mut qf = match File::open(a.path()) {
                Ok(f) => f,
                Err(_) => return Err(ExamshError::OpenFile(a.path())),
            };

            let mut qcontent = String::new();
            match qf.read_to_string(&mut qcontent) {
                Err(_) => return Err(ExamshError::ReadFile(a.path())),
                Ok(c) => c,
            };

            let q: Question = match serde_json::from_str(&qcontent) {
                Err(_) => return Err(ExamshError::ParseExamFile("question".into(), a.path())),
                Ok(q) => q,
            };

            let question: Question = match q {
                Question::PredictOutputQuestion(mut predict) => {
                    predict.parse_code(&questions_path);
                    Question::PredictOutputQuestion(predict)
                }

                _ => q,
            };
            questions.push(question);
        }

        questions.sort_by_key(|a| a.get_question_id());

        let mut exam_dir = fname;
        exam_dir.pop();
        Ok(Exam {
            exam_dir,
            exam_schema,
            questions,
        })
    }

    pub fn make_exam(&self) -> Result<(), ExamshError> {
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

        render_latex("exam.tex".into(), &self.exam_dir, &exam)?;
        render_latex("marking.tex".into(), &self.exam_dir, &marking)
    }

    fn generate_questions(&self) -> String {
        self.questions
            .iter()
            .map(|q| q.render())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
