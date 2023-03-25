use crate::utils::{exec_shell, wrap_in_code_blocks};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PredictOutput {
    pub question: String,
    pub pre_run: String,
    pub run: String,
    pub post_run: String,

    // PathBuf of the file. If second arg is None, load from disk. otherwise use content
    pub _code: Vec<(String, Option<String>)>,

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

        if !self.pre_run.is_empty() {
            exec_shell(&self.pre_run, &temp_dir);
        }

        let output = exec_shell(&self.run, &temp_dir);

        if !self.post_run.is_empty() {
            exec_shell(&self.post_run, &temp_dir);
        }

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
