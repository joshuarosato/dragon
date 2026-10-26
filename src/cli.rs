use std::{
    env,
    ffi::OsString,
    io::{self, IsTerminal, Read},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Cli {
    pub paths: Vec<PathBuf>,
    pub keep_open: bool,
}

#[derive(Debug)]
pub enum CliExit {
    Success,
    Failure(String),
}

impl Cli {
    pub fn parse() -> Result<Self, CliExit> {
        let mut keep_open = false;
        let mut parsing_options = true;
        let mut read_stdin = false;
        let mut paths = Vec::new();

        for arg in env::args_os().skip(1) {
            if parsing_options {
                if arg == OsString::from("--") {
                    parsing_options = false;
                    continue;
                }

                if arg == OsString::from("-h") || arg == OsString::from("--help") {
                    print_help();
                    return Err(CliExit::Success);
                }

                if arg == OsString::from("-V") || arg == OsString::from("--version") {
                    println!("dragon {}", env!("CARGO_PKG_VERSION"));
                    return Err(CliExit::Success);
                }

                if arg == OsString::from("-k") || arg == OsString::from("--keep") {
                    keep_open = true;
                    continue;
                }

                if arg == OsString::from("-") {
                    read_stdin = true;
                    continue;
                }

                if arg.to_string_lossy().starts_with('-') {
                    return Err(CliExit::Failure(format!(
                        "unknown option: {}",
                        arg.to_string_lossy()
                    )));
                }
            }

            paths.push(PathBuf::from(arg));
        }

        if read_stdin || (paths.is_empty() && !io::stdin().is_terminal()) {
            paths.extend(paths_from_stdin()?);
        }

        if paths.is_empty() {
            return Err(CliExit::Failure("no files specified".into()));
        }

        let mut resolved = Vec::with_capacity(paths.len());
        for path in paths {
            match path.canonicalize() {
                Ok(path) => resolved.push(path),
                Err(error) => {
                    return Err(CliExit::Failure(format!(
                        "{}: {error}",
                        path.display()
                    )));
                }
            }
        }

        Ok(Self {
            paths: resolved,
            keep_open,
        })
    }
}

fn paths_from_stdin() -> Result<Vec<PathBuf>, CliExit> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    stdin
        .read_to_string(&mut input)
        .map_err(|error| CliExit::Failure(format!("failed to read stdin: {error}")))?;

    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect())
}

fn print_help() {
    println!(
        "dragon {version}\n\
         Drag files from the command line into GUI applications.\n\n\
         USAGE:\n    dragon [OPTIONS] [FILE...]\n\n\
         FILES:\n    -                Read newline-delimited file paths from stdin\n\n\
         OPTIONS:\n    -k, --keep       Keep the window open after a successful drop\n    -h, --help       Print help\n    -V, --version    Print version\n\n\
         If no FILE arguments are given and stdin is piped, paths are read from stdin.\n\n\
         EXAMPLES:\n    dragon myfile.zip\n    dragon image.png report.pdf\n    printf '%s\\n' image.png report.pdf | dragon\n    printf '%s\\n' report.pdf | dragon image.png -\n    dragon --keep *.jpg\n    dragon -- --filename-starting-with-a-dash",
        version = env!("CARGO_PKG_VERSION")
    );
}
