#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::PathBuf;

use app::{exam::{Exam, ExamSchema}, errors::ExamshError};

#[tauri::command]
fn load_exam(directory: String) -> Result<Exam, ExamshError> {
    let path_buf = PathBuf::from(directory);
    if !path_buf.is_dir() {
        Err(ExamshError::NotDirectory(path_buf))
    } else {
        let mut d = path_buf;
        d.push("exam.json");
        if !d.exists() {
            return Err(ExamshError::NoExamFileFound(d));
        }

        Exam::from_exam_file(d)

    }

}

#[tauri::command]
fn create_new_exam(directory: String, 
                   exam_schema: ExamSchema) -> Result<(), ExamshError> {
    println!("Name: {}",exam_schema.test_name);
    println!("Directory: {}", directory);
    let path_buf = PathBuf::from(directory);
    if !path_buf.is_dir() {
        Err(ExamshError::NotDirectory(path_buf))
    } else {
        match check_if_directory_empty(&path_buf){
            Ok(b) => {
                if b {
                     Exam::create_exam(exam_schema, &path_buf)
                } else {
                    Err(ExamshError::DirectoryNotEmpty(path_buf))
                }
            },
            Err(e) =>Err(e )
        }
    }
}

fn check_if_directory_empty(path_buf: &PathBuf) -> Result<bool, ExamshError> {
    match path_buf.read_dir() {
        Err(_) => Err(ExamshError::Unexpected(format!("Unable to read directory: {}", path_buf.display()))),
        Ok(mut iter) => {
            Ok(iter.next().is_none())
        }
    }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![create_new_exam,load_exam])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
