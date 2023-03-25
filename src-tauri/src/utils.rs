use std::{
    fs::OpenOptions,
    io::Write,
    path::{PathBuf, Path},
    process::{Command, Stdio},
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

pub fn render_latex(
    latexfname: String,
    out_dir: &PathBuf,
    content: &str,
) -> Result<(), ExamshError> {
    let mut latexname = out_dir.clone();
    latexname.push(latexfname);

    match which("pdflatex") {
        Ok(_) => (),
        Err(e) => {
            return Err(ExamshError::Unexpected(format!(
                "Could not find pdflatex: {}",
                e
            )))
        }
    }

    let mut f = match OpenOptions::new().create(true).write(true).open(&latexname) {
        Err(_) => return Err(ExamshError::OpenFile(latexname)),
        Ok(f) => f,
    };

    if write!(f, "{}", content).is_err() {
        return Err(ExamshError::WriteFile(latexname));
    }

    if f.flush().is_err() {
        return Err(ExamshError::Unexpected(format!(
            "Unable to flush file: {}",
            latexname.display()
        )));
    }

    match Command::new("pdflatex")
        .arg("-output-directory")
        .arg(out_dir)
        .arg(format!("\"{}\"", latexname.display()))
        .stdout(Stdio::null())
        .status()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(ExamshError::Unexpected(err.to_string())),
    }
}

#[cfg(unix)]
pub fn exec_shell(cmd: &String, current_dir: &Path) -> String  {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(current_dir)
            .output()
            .expect(&format!("Unable to run {}", cmd));

        String::from_utf8(output.stdout).expect("Unable to get output")

}


#[cfg(windows)]
pub fn exec_shell(cmd: &String, current_dir: &Path) -> String  {
    let comspec = std::env::var_os("COMSPEC").unwrap_or_else(|| "cmd.exe".into());
    let output = Command::new(comspec)
        .arg("/C")
        .arg(cmd)
        .current_dir(current_dir)
        .output()
        .expect(&format!("Unable to run {}", cmd));

    String::from_utf8(output.stdout).expect("Unable to get output")
}
