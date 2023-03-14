use std::{
    fs::OpenOptions,
    io::Write,
    process::{Command, Stdio},
};

pub fn wrap_in_code_blocks(s: &str) -> String {
    format!(
        "\\begin{{verbatim}}
{}
\\end{{verbatim}}",
        s
    )
}

pub fn render_latex(latexname: &str, out_dir: &str, content: &str) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(latexname)
        .expect("Unable to open file for writing.");
    write!(f, "{}", content).expect("Unable to write to file.");
    f.flush().expect("Unable to flush.");

    Command::new("pdflatex")
        .arg("-output-directory")
        .arg(out_dir)
        .arg(format!("\"{}\"", latexname))
        .stdout(Stdio::null())
        .status()
        .expect("Unable to execute latex renderer");
}
