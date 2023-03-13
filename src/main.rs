use chrono::Utc;
use std::{fs::OpenOptions, io::Write, path::PathBuf, process::Command};

const QPFILE: &str = "/tmp/exam.tex";
const MSFILE: &str = "/tmp/marking.tex";
const OUT_DIR: &str = "/tmp";

const INSTRUCTIONS: &str = "Answer the questions in the rust programming language.
Make sure your program follows the input and output
text given in the question.";

const COURSE: &str = "EXAMSH";
const TEST_NAME: &str = "Basic Test";

struct PredictOutput<'a> {
    question: &'a str,
    code: Vec<(&'a str, &'a str)>,
    run_cmd: fn(PathBuf) -> Result<String, String>,
}

impl<'a> PredictOutput<'a> {
    fn render(&self) -> String {
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
                println!("done");
                f.flush().expect("Unable to flush");

                format!(
                    "
\\textbf{{{}}}
\\begin{{verbatim}}
{}
\\end{{verbatim}}
",
                    fname, code
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let output = (self.run_cmd)(temp_dir);
        if output.is_err() {
            eprintln!("[CODE RUNNER] Error occured.");
            "ERROR".to_string()
        } else {
            format!(
                "
\\question
{}

Code:

{}

\\begin{{solution}}
\\begin{{verbatim}}
{}
\\end{{verbatim}}
\\end{{solution}}
",
                self.question,
                code,
                output.unwrap()
            )
        }
    }
}

struct MultipleChoiceQuestions<'a> {
    question: &'a str,
    answers: Vec<&'a str>,
    correct_id: usize,
}

impl<'a> MultipleChoiceQuestions<'a> {
    fn render(&self) -> String {
        let choices = self
            .answers
            .iter()
            .enumerate()
            .map(|(id, ans)| {
                let ch = if self.correct_id == id {
                    "\\correctchoice "
                } else {
                    "\\choice "
                };

                format!("{} {}", ch, ans)
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "
\\question
{}
\\begin{{choices}}
{}
\\end{{choices}}
        ",
            self.question, choices
        )
    }
}

struct WriteCode<'a> {
    question: &'a str,
    output: &'a str,
}

impl<'a> WriteCode<'a> {
    fn render(&self) -> String {
        format!(
            "
\\question
{}

The output should exactly match what is given below:
\\begin{{verbatim}}
{}
\\end{{verbatim}}
            ",
            self.question, self.output
        )
    }
}

fn generate_questions() -> String {
    let q1 = MultipleChoiceQuestions {
        question: "What language is \\verb|examsh| written in ?",
        answers: vec!["C Plus Plus", "Rust"],
        correct_id: 1,
    }
    .render();
    let q2 = WriteCode {
        question: "Write a program to convert between the various temperature ranges (\\degree C, \\degree F and Kelvin).",
        output: "
Temperature Converter
Enter the scale you want to convert FROM (K, F, C): K
Enter the temperature in Kelvin: 300
Enter the scale you want to convert TO (F, C): C
300 K is 26.85 C"
    }.render();
    let q3 = PredictOutput {
        question: "Find out output",
        code: vec![(
            "main.rs",
            "
fn main() {
    println!(\"Hello World!\");
}
",
        )],
        run_cmd: |a| {
            let mut path = a.clone();
            path.push("main.rs");

            Command::new("rustc")
                .arg(path.to_string_lossy().to_string())
                .arg(format!("--out-dir={}", a.to_string_lossy()))
                .status()
                .expect("Unable to compile rust code.");

            path.pop();
            path.push("main");

            let o = Command::new(path).output().expect("Unable to run command");

            let s = String::from_utf8(o.stdout).expect("Unexpected output from code.");
            Ok(s)
        },
    }
    .render();
    format!("{}\n{}\n{}", q1, q2, q3)
}

fn render_latex(latexname: &str, out_dir: &str, content: &str) {
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
        .status()
        .expect("Unable to execute latex renderer");
}

fn main() {
    println!("examsh");

    let today = Utc::now().format("%d %B %Y").to_string();

    let d = include_str!("./base_doc.tex")
        .replace("COURSE", COURSE)
        .replace("TEST_NAME", TEST_NAME)
        .replace("INSTRUCTIONS", INSTRUCTIONS)
        .replace("DATE", &today)
        .replace("QUESTIONS", generate_questions().as_str());

    let exam = d.replace("MODE", "12pt, addpoints");
    let marking = d.replace("MODE", "12pt, answers");
    render_latex(QPFILE, OUT_DIR, &exam);
    render_latex(MSFILE, OUT_DIR, &marking);
}
