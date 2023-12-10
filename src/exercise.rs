use regex::Regex;
use serde::Deserialize;

use std::fmt::{self, Display, Formatter};
use std::fs::{remove_file, write, File};
use std::io::Read;
use std::path::PathBuf;
use std::process;

use crate::scarb::{scarb_build, scarb_run, scarb_test};

const I_AM_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Build,
    Run,
    Test,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize, Debug)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: Mode,
    pub hint: String,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Done,
    Pending(Vec<ContextLine>),
}

#[derive(PartialEq, Debug)]
pub struct ContextLine {
    pub line: String,
    pub number: usize,
    pub important: bool,
}

#[derive(Debug)]
pub struct ExerciseOutput {
    pub stdout: String,
    pub stderr: String,
}

struct FileHandle;

impl Drop for FileHandle {
    fn drop(&mut self) {
        clean();
    }
}

impl Exercise {
    pub fn build(&self) -> anyhow::Result<String> {
        scarb_build(&self.path)
    }

    pub fn run(&self) -> anyhow::Result<String> {
        scarb_run(&self.path)
    }

    pub fn test(&self) -> anyhow::Result<String> {
        scarb_test(&self.path)
    }

    pub fn state(&self) -> State {
        let mut source_file = File::open(&self.path).unwrap_or_else(|_| {
            panic!("We were unable to open the exercise file! {:?}", self.path)
        });

        let source = {
            let mut s = String::new();
            source_file
                .read_to_string(&mut s)
                .expect("We were unable to read the exercise file!");
            s
        };

        let re = Regex::new(I_AM_DONE_REGEX).unwrap();

        if !re.is_match(&source) {
            return State::Done;
        }

        let matched_line_index = source
            .lines()
            .enumerate()
            .find_map(|(i, line)| if re.is_match(line) { Some(i) } else { None })
            .expect("This should not happen at all");

        let min_line = ((matched_line_index as i32) - (CONTEXT as i32)).max(0) as usize;
        let max_line = matched_line_index + CONTEXT;

        let context = source
            .lines()
            .enumerate()
            .filter(|&(i, _)| i >= min_line && i <= max_line)
            .map(|(i, line)| ContextLine {
                line: line.to_string(),
                number: i + 1,
                important: i == matched_line_index,
            })
            .collect();

        State::Pending(context)
    }

    pub fn looks_done(&self) -> bool {
        self.state() == State::Done
    }

    pub fn mark_done(&self) -> anyhow::Result<()> {
        let mut source_file = File::open(&self.path)?;
        let mut source = String::new();
        source_file.read_to_string(&mut source)?;

        let updated_source = source.replace(regex::Regex::new(I_AM_DONE_REGEX)?, "");
        write(&self.path, updated_source)?;

        Ok(())
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

pub fn create_new_exercise(name: &str, path: &str, mode: Mode, hint: &str) -> Exercise {
    Exercise {
        name: name.to_string(),
        path: PathBuf::from(path),
        mode,
        hint: hint.to_string(),
    }
}

pub fn display_exercise_info(exercise: &Exercise) {
    println!("Exercise: {}", exercise.name);
    println!("Path: {:?}", exercise.path);
    println!("Mode: {:?}", exercise.mode);
    println!("Hint: {}", exercise.hint);
}

pub fn display_exercise_state(exercise: &Exercise) {
    match exercise.state() {
        State::Done => println!("Exercise '{}' is marked as DONE.", exercise.name),
        State::Pending(context) => {
            println!("Exercise '{}' is NOT DONE. Context:", exercise.name);
            for line in context {
                let marker = if line.important { "*" } else { "" };
                println!("{} {}: {}", marker, line.number, line.line);
            }
        }
    }
}

#[inline]
fn clean() {
    let _ignored = remove_file(temp_file());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finished_exercise() {
        let exercise = Exercise {
            name: "finished_exercise".into(),
            path: PathBuf::from("tests/fixture/cairo/compilePass.cairo"),
            mode: Mode::Build,
            hint: String::new(),
        };

        assert_eq!(exercise.state(), State::Done);
    }

    #[test]
    fn test_cairo_test_passes() {
        let exercise = Exercise {
            name: "testPass".into(),
            path: PathBuf::from("tests/fixture/cairo/testPass.cairo"),
            mode: Mode::Build,
            hint: String::new(),
        };

        assert_eq!(exercise.state(), State::Done);
    }
}
