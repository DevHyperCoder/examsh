#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{path::PathBuf, collections::HashMap, sync::{Mutex, Arc}};

use app::{exam::{Exam, ExamSchema}, errors::ExamshError};

struct LoadedExams {
    pub current_exam: String,
    pub loaded: HashMap<String, Exam>
}

#[tauri::command]
fn load_exam_with_ident(exam_ident: String, state:tauri::State<Arc<Mutex<LoadedExams>>>) -> Result<Exam, ExamshError> {
    let exam_path = state.lock().unwrap().loaded.get(&exam_ident).cloned();
    match exam_path {
        None => {
            return Err(ExamshError::NotInCache())
        },
        Some(e) => Ok(e)
    }
}

#[tauri::command]
fn load_exam(directory: String,  state: tauri::State<Arc<Mutex<LoadedExams>>>) -> Result<(Exam, String), ExamshError> {
    let path_buf = PathBuf::from(directory);
    if !path_buf.is_dir() {
        Err(ExamshError::NotDirectory(path_buf))
    } else {
        let mut d = path_buf;
        d.push("exam.json");
        if !d.exists() {
            return Err(ExamshError::NoExamFileFound(d));
        }


        let res =  Exam::from_exam_file(d.clone());

        if res.is_ok() {
            let exam = res.as_ref().expect("PANIC");
            state.lock().unwrap().loaded.insert(exam.get_identifier(), exam.to_owned());
            state.lock().unwrap().current_exam = exam.get_identifier();
        }
        match res {
            Err(e) => Err(e),
            Ok(exam) => Ok((exam.clone(), exam.get_identifier()))
        }
    }

}

#[tauri::command]
fn create_new_exam(directory: String, 
                   exam_schema: ExamSchema,
                   state: tauri::State<Arc<Mutex<LoadedExams>>>) -> Result<Exam, ExamshError> {
    println!("Name: {}",exam_schema.test_name);
    println!("Directory: {}", directory);
    let path_buf = PathBuf::from(directory);
    if !path_buf.is_dir() {
        Err(ExamshError::NotDirectory(path_buf))
    } else {
        match check_if_directory_empty(&path_buf){
            Ok(b) => {
                if b {
                     let res = Exam::create_exam(exam_schema, &path_buf);
                     if res.is_ok() {
                                let exam = res.as_ref().expect("PANIC");
                                state.lock().unwrap().loaded.insert(exam.get_identifier(), exam.to_owned());
                                state.lock().unwrap().current_exam = exam.get_identifier();

                     }
                     res
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
      .manage(Arc::new(Mutex::new(LoadedExams{
          current_exam: "".into(),
          loaded: HashMap::new()
      })))
      .invoke_handler(tauri::generate_handler![create_new_exam,load_exam,load_exam_with_ident])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
