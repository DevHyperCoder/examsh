use std::{
    fs::OpenOptions,
    io::Write,
    process::{Command, Stdio}, path::PathBuf,
};

use which::which;

use crate::errors::ExamshError;

pub fn wrap_in_code_blocks(s: &str) -> String {
    format!(
        "\\begin{{verbatim}}
{}
\\end{{verbatim}}",
        s
    )
}

pub fn render_latex(latexfname: String, out_dir: &PathBuf, content: &str) -> Result<(), ExamshError> {
    let mut latexname = out_dir.clone();
    latexname.push(latexfname);

    match which("pdflatex") {
        Ok(_) => (),
        Err(e) => {
            return Err(ExamshError::Unexpected(format!("Could not find pdflatex: {}",e.to_string())))
        }
    }

    let mut f = match OpenOptions::new()
        .create(true)
        .write(true)
        .open(&latexname) {
            Err(_) => return Err(ExamshError::OpenFile(latexname)),
            Ok(f) => f
    };

    match write!(f, "{}", content) {
        Err(_) => return Err(ExamshError::WriteFile(latexname)),
        Ok(_) => ()
    };

    match f.flush() {
        Err(_) => return Err(ExamshError::Unexpected(format!("Unable to flush file: {}", latexname.display()))),
        Ok(_) => ()
    };

    match Command::new("pdflatex")
        .arg("-output-directory")
        .arg(out_dir)
        .arg(format!("\"{}\"", latexname.display()))
        .stdout(Stdio::null())
        .status() {
            Ok(_) => {Ok(())},
            Err(err) => {
                Err(ExamshError::Unexpected(err.to_string()))

            }
    }

}
