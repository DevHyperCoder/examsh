use std::{path::PathBuf, fmt::Display};

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ExamshError {
    OpenFile(PathBuf),
    CreateFile(PathBuf),
    WriteFile(PathBuf),
    ReadFile(PathBuf),

    DirectoryNotEmpty(PathBuf),
    NotDirectory(PathBuf),

    NoExamFileFound(PathBuf),

    ParseExamFile(String,PathBuf),

    NotInCache(),

    Unexpected(String),
}

impl Display for ExamshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ExamshError::OpenFile(path) => format!("Unable to open file: {}", path.display()),
            ExamshError::CreateFile(path) => format!("Unable to create file: {}", path.display()),
            ExamshError::ReadFile(path) => format!("Unable to read file: {}", path.display()),
            ExamshError::WriteFile(path) => format!("Unable to write file: {}", path.display()),

            ExamshError::DirectoryNotEmpty(path) => format!("Directory not empty: {}", path.display()),
            ExamshError::NotDirectory(path) => format!("Not a directory: {}", path.display()),

            ExamshError::ParseExamFile(ftype,path) => format!("Unable to parse {} file: {}",ftype, path.display()),

            ExamshError::NoExamFileFound(path) => format!("No exam file found at {}", path.display()),

            ExamshError::NotInCache() => format!("Exam was not found."),

            ExamshError::Unexpected(msg) => format!("Unexpected error: {}", msg),

        })
    }
}


impl serde::Serialize for ExamshError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

