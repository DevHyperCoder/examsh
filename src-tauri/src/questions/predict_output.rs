use crate::{questions::Question, utils::wrap_in_code_blocks};

use chrono::Utc;
use serde::{Serialize,Deserialize};
use std::{fs::OpenOptions, io::Write, process::Command};

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct PredictOutput {
    pub question: String,
    pub pre_run: String,
    pub run: String,
    pub post_run: String,
    pub code_files: Vec<String>,

    #[serde(skip)]
    pub code: Vec<(String, String)>,
}

impl PredictOutput {
    pub fn render(&self) -> String {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push(format!("examsh-{}", Utc::now()));
        std::fs::create_dir(&temp_dir).expect("Unable to create temp directory");

        let code = &self
            .code
            .iter()
            .map(|(fname, code)| {
                let mut asdf = temp_dir.clone();
                asdf.push(fname);
                let mut f = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(asdf)
                    .expect("Unable to open file");
                write!(f, "{}", code).expect("Unable to write to file");
                f.flush().expect("Unable to flush");

                format!(
                    "
\\textbf{{{}}}
{}
",
                    fname,
                    wrap_in_code_blocks(code)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        // TODO: The command runner is specific to *nix (atleast those that have /bin/sh).
        // Need to figure out a way to make it platform indep.

        Command::new("/bin/sh")
            .arg("-c")
            .arg(&self.pre_run)
            .current_dir(&temp_dir)
            .status()
            .expect("Unable to run pre_cmd");

        let output = Command::new(&self.run)
            .current_dir(&temp_dir)
            .output()
            .expect("Unable to run program");
        let output = String::from_utf8(output.stdout).expect("Unable to get output");

        Command::new("/bin/sh")
            .arg("-c")
            .arg(&self.post_run)
            .current_dir(&temp_dir)
            .status()
            .expect("Unable to run post_cmd");

        format!(
            "
\\question
{}

Code:

{}

\\begin{{solution}}
{}
\\end{{solution}}
",
            self.question,
            code,
            wrap_in_code_blocks(output.as_str())
        )
    }
}
