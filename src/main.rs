use std::{path::PathBuf, process::Command, fmt::{Display, Write}, fs};

use glob::{PatternError, glob};

struct File {
    path: PathBuf,
    contributors: Vec<String>,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self.path)?;
        for contributor in self.contributors.iter() {
            write!(f, "{contributor},")?;
        }
        Ok(())
    }
}

impl File {
    fn fetch(path: PathBuf) -> Option<Self> {
        let output = Command::new("git").arg("log").arg(format!("--pretty='%aN'")).arg(&path).output().ok()?;
        let output = String::from_utf8(output.stdout).ok()?;
        let mut contributors: Vec<_> = output.lines().collect();
        contributors.sort();
        contributors.dedup();

        Some(Self {
            path,
            contributors: contributors.into_iter().map(String::from).collect(),
        })
    }
}

fn find_format(format: &str) -> Result<Vec<File>, PatternError> {
    let pattern = format!("**/*.{format}");
    glob(&pattern).map(|paths| {
        paths.into_iter().filter_map(Result::ok).filter_map(File::fetch).collect()
    })
}

fn main() {
    let formats = ["ico", "bin"];
    for format in formats {
        let v = find_format(format).unwrap();
        let mut content = String::new();
        for f in v {
            writeln!(&mut content, "{f}").unwrap();
        }
        fs::write(format!("result.{format}.txt"), content.as_bytes()).expect("Couldn't write to file");
        println!("Finnished .{format}");
    }
}
