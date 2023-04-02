use crate::utils::{exec_shell, wrap_in_code_blocks};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

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

    pub question_id: String,
}

impl PredictOutput {
    pub fn get_id(&self) -> String {
        self.question_id.to_owned()
    }
    pub fn parse_code(&mut self, questions_path: &Path) {
        let codes = self
            ._code
            .iter()
            .map(|(code_fname, code)| match code {
                Some(s) => (code_fname.to_string(), s.to_string()),
                None => {
                    let mut asdf = questions_path.to_path_buf();
                    asdf.push(code_fname);
                    let mut f = File::open(asdf).expect("Unable to open code file.");
                    let mut fc = String::new();
                    f.read_to_string(&mut fc).expect("Unable to read file");
                    (code_fname.to_string(), fc)
                }
            })
            .collect::<Vec<(String, String)>>();

        self.code = codes;
    }

    pub fn render(&self) -> String {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push(format!("examsh-{}", Utc::now()));
        std::fs::create_dir(&temp_dir).expect("Unable to create temp directory");

        println!("{:?}", &self.code);
        let code = &self
            .code
            .iter()
            .map(|(fname, code)| {
                let mut asdf = temp_dir.clone();
                asdf.push(fname);
                println!("{}", asdf.display());
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
