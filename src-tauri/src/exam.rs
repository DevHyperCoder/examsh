use std::{
    fs::{self, File, OpenOptions},
    io::{Read,Write},
    path::PathBuf,
};

use chrono::Utc;
use serde::{Serialize,Deserialize};

use crate::{
    questions::Question,
    utils::render_latex,
    errors:: ExamshError
};

const QPFILE: &str = "/tmp/exam.tex";
const MSFILE: &str = "/tmp/marking.tex";
const OUT_DIR: &str = "/tmp";

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ExamSchema {
    pub course_name: String,
    pub test_name: String,
    pub instructions: String,
    pub marking_instructions: String,
    pub date_fmt: Option<String>,
}

#[derive(Deserialize,Serialize)]
enum QuestionType {
    MultipleChoiceQuestion,
    PredictOutput,
    WriteCode,
    Raw,
}

#[derive(Clone, Debug,Serialize)]
pub struct Exam {
    exam_schema: ExamSchema,
    exam_dir: PathBuf,
    questions: Vec<Question>,
}

impl Exam {
    pub fn get_questions(&self) -> &Vec<Question> {
        &self.questions
    }
    pub fn add_question(&mut self, question: Question) -> Result<&mut Exam, ExamshError> {
        let mut newf = self.exam_dir.clone();
        newf.push("questions");
        newf.push(format!("examsh-{}.json",Utc::now()));

        let mut f = match OpenOptions::new()
            .create(true)
            .write(true)
            .open(&newf) {
                Err(e) => {println!("{:?}",e ); return Err(ExamshError::CreateFile(newf))},
                Ok(f) => f
            };
        let s = match serde_json::to_string_pretty(&question) {
            Err(_) => return Err(ExamshError::Unexpected("Unable to make question".into())),
            Ok(s) => s
        };

        match write!(f, "{}", s) {
            Ok(_) => {
                self.questions.push(question);
                Ok(self)},
            Err(_) => Err(ExamshError::WriteFile(newf))
        }


    }
    pub fn get_identifier(&self) -> String {
        format!("{}${}",self.exam_schema.course_name, self.exam_schema.test_name)
    }
    pub fn create_exam(exam_schema: ExamSchema, dir: &PathBuf) -> Result<Exam, ExamshError> {
        let mut d = dir.clone();
        d.push("exam.json");
        match File::create(&d) {
            Err(_) => Err(ExamshError::CreateFile(d)),
            Ok(mut f) => {

                
                match serde_json::to_string_pretty(&exam_schema) {
                    Ok(s) => 
                        match write!(f,"{}", s) {
                            Ok(_) => {
                                d.pop();
                                d.push("questions");
                                match fs::create_dir(&d) {
                                    Err(_) => Err(ExamshError::Unexpected(format!("Unable to create questions directory at: {}", d.display()))),
                                    Ok(_) => Ok(Exam {
                                        exam_dir: d,
                                        questions: vec![],
                                        exam_schema
                                    })
                                }

                            }
                            Err(_) => {
                                Err(ExamshError::WriteFile(d))
                            }
                    }, Err(_) => {
                        Err(ExamshError::Unexpected(format!("Unable to make exam file.")))
                    }
                }
            }
        }
    }
    pub fn from_exam_file(fname: PathBuf) -> Result<Exam,ExamshError> {
        let mut f = match File::open(&fname){
Ok(f) => f,
Err(_) => {
    return Err(ExamshError::OpenFile(fname))
}};

        let mut content = String::new();
        match f.read_to_string(&mut content) {
            Err(_) => {return Err(ExamshError::ReadFile(fname))},
            Ok(_) => {}
        };

        let exam_schema: ExamSchema =
        match
            serde_json::from_str(&content) {
                Err(e) => {
                    println!("{:?}", e);
                    return Err(ExamshError::ParseExamFile("exam".into(), fname))
                },
                Ok(s) => s
            };

        // TODO This code makes an assumptions that the questions for this exam are present in a
        // subfolder called "questions".
        // - Only 1 level deep questions are supported and that too for PredictOutput questions.
        // - recursive questions are not supported.

        let mut questions_path = PathBuf::from(&fname);
        questions_path.pop(); // pop file name
        questions_path.push("questions");

        let mut questions = vec![];
        let dir_content =match fs::read_dir(&questions_path) {
            Err(_) => {return Err(ExamshError::Unexpected(format!("Unable to read questions directory: {}", questions_path.display())))}
            Ok(e) => e
        };

        for dir in dir_content {
            let a = match dir{
                Err(_) => return Err(ExamshError::Unexpected(format!("Unable to get directory entires of {}", questions_path.display()))),
                Ok(a) => a
            };
            if !a.path().is_file() {
                continue;
            }
            let path = a.path();
            let e = path.extension();
            let ext = match  e{
                Some(ext) => ext, 
                None => return Err(ExamshError::Unexpected(format!("Can not get file extension for {}", a.path().to_string_lossy())))
            };
            if ext  != "json" {
                continue;
            }

            let mut qf = match File::open(a.path()) {Ok(f) => f, Err(_) => return Err(ExamshError::OpenFile(a.path()))};

            let mut qcontent = String::new();
            match qf.read_to_string(&mut qcontent) {
                Err(_) => return Err(ExamshError::ReadFile(a.path())),
                Ok(c) => c
            };

            let q:Question = match serde_json::from_str(&qcontent) {
                Err(_) => return Err(ExamshError::ParseExamFile("question".into(), a.path())),
                Ok(q) => q
            };
            let question: Question = match q  {
                Question::PredictOutputQuestion(mut predict) => {
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
                    Question::PredictOutputQuestion(predict)
                }

                _ => q
            };
            questions.push(question);
        }

        let mut exam_dir = PathBuf::from(fname);
        exam_dir.pop();
        Ok(Exam {
            exam_dir,
            exam_schema,
            questions,
        })
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
