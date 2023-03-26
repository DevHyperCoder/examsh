#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use app::{
    errors::ExamshError,
    exam::{Exam, ExamSchema},
    questions::Question,
};

struct LoadedExams {
    pub current_exam: String,
    pub loaded: HashMap<String, Exam>,
}

#[tauri::command]
fn render_exam(
    exam_ident: String,
    state: tauri::State<Arc<Mutex<LoadedExams>>>,
) -> Result<(), ExamshError> {
    let s = state.lock().unwrap();
    let exam = s.loaded.get(&exam_ident);
    let exam = match exam {
        None => return Err(ExamshError::NotInCache()),
        Some(e) => e,
    };

    exam.make_exam()
}

#[tauri::command]
fn add_question(
    exam_ident: String,
    question: Question,
    state: tauri::State<Arc<Mutex<LoadedExams>>>,
) -> Result<Vec<Question>, ExamshError> {
    let mut s = state.lock().unwrap();
    let exam = s.loaded.get_mut(&exam_ident);
    let exam = match exam {
        None => return Err(ExamshError::NotInCache()),
        Some(e) => e,
    };

    match exam.add_question(question) {
        Err(e) => Err(e),
        Ok(exam) => Ok(exam.clone().get_questions().clone()),
    }
}

#[tauri::command]
fn load_exam_with_ident(
    exam_ident: String,
    state: tauri::State<Arc<Mutex<LoadedExams>>>,
) -> Result<Exam, ExamshError> {
    let exam_path = state.lock().unwrap().loaded.get(&exam_ident).cloned();
    match exam_path {
        None => Err(ExamshError::NotInCache()),
        Some(e) => Ok(e),
    }
}

#[tauri::command]
fn load_exam(
    directory: String,
    state: tauri::State<Arc<Mutex<LoadedExams>>>,
) -> Result<(Exam, String), ExamshError> {
    let path_buf = PathBuf::from(directory);
    if !path_buf.is_dir() {
        Err(ExamshError::NotDirectory(path_buf))
    } else {
        let mut d = path_buf;
        d.push("exam.json");
        if !d.exists() {
            return Err(ExamshError::NoExamFileFound(d));
        }

        let res = Exam::from_exam_file(d.clone());

        if res.is_ok() {
            let exam = res.as_ref().expect("PANIC");
            state
                .lock()
                .unwrap()
                .loaded
                .insert(exam.get_identifier(), exam.to_owned());
            state.lock().unwrap().current_exam = exam.get_identifier();
        }
        match res {
            Err(e) => Err(e),
            Ok(exam) => Ok((exam.clone(), exam.get_identifier())),
        }
    }
}

#[tauri::command]
fn create_new_exam(
    directory: String,
    exam_schema: ExamSchema,
    state: tauri::State<Arc<Mutex<LoadedExams>>>,
) -> Result<(Exam, String), ExamshError> {
    let path_buf = PathBuf::from(directory);
    if !path_buf.is_dir() {
        Err(ExamshError::NotDirectory(path_buf))
    } else {
        match check_if_directory_empty(&path_buf) {
            Ok(b) => {
                if b {
                    let res = Exam::create_exam(exam_schema, &path_buf);
                    match res {
                        Ok(exam) => {
                            state
                                .lock()
                                .unwrap()
                                .loaded
                                .insert(exam.get_identifier(), exam.to_owned());
                            state.lock().unwrap().current_exam = exam.get_identifier();
                            Ok((exam.to_owned(), exam.get_identifier()))
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err(ExamshError::DirectoryNotEmpty(path_buf))
                }
            }
            Err(e) => Err(e),
        }
    }
}

fn check_if_directory_empty(path_buf: &Path) -> Result<bool, ExamshError> {
    match path_buf.read_dir() {
        Err(_) => Err(ExamshError::Unexpected(format!(
            "Unable to read directory: {}",
            path_buf.display()
        ))),
        Ok(mut iter) => Ok(iter.next().is_none()),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(LoadedExams {
            current_exam: "".into(),
            loaded: HashMap::new(),
        })))
        .invoke_handler(tauri::generate_handler![
            create_new_exam,
            load_exam,
            load_exam_with_ident,
            add_question,
            render_exam
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
