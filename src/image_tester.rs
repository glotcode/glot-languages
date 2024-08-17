use crate::language::Language;
use crate::language::RunInstructions;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::string;

pub enum Test {
    HelloWorld,
    Version,
}

impl Test {
    pub fn run(&self, language: Language) -> Result<RunResult, Error> {
        match self {
            Test::HelloWorld => run_hello_world(language),
            Test::Version => run_version(language),
        }
    }
}

pub fn test_images(test: Test, languages: Vec<Language>) {
    for language in languages {
        let result = test.run(language);

        match result {
            Ok(run_result) => print_success(language, run_result),
            Err(error) => print_failure(language, error),
        }
    }
}

fn run_hello_world(language: Language) -> Result<RunResult, Error> {
    let run_request = prepare_hello_request(language);
    let run_result = run_container(language, run_request)?;
    check_hello_result(&run_result)?;
    Ok(run_result)
}

fn run_version(language: Language) -> Result<RunResult, Error> {
    let run_request = prepare_version_request(language);
    let run_result = run_container(language, run_request)?;
    check_version_result(&run_result)?;
    Ok(run_result)
}

fn run_container(language: Language, run_request: RunRequest) -> Result<RunResult, Error> {
    let run_config = language.config().run_config();
    let command = format!("docker run --pull never --rm -i --read-only --tmpfs /tmp:rw,noexec,nosuid,size=65536k --tmpfs /home/glot:rw,exec,nosuid,uid=1000,gid=1000,size=131072k -u glot -w /home/glot {}", run_config.container_image);
    let stdin = serde_json::to_string(&run_request).map_err(Error::SerializeRequest)?;

    let options = Options { command, stdin };
    let cmd_output = run(options)?;

    get_run_result(cmd_output)
}

fn get_run_result(cmd_output: SuccessOutput) -> Result<RunResult, Error> {
    if !cmd_output.stderr.is_empty() {
        Err(Error::NonEmptyStderr(cmd_output.stderr))
    } else {
        serde_json::from_str(&cmd_output.stdout).map_err(Error::DeserializeResult)
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    stdout: String,
    stderr: String,
    error: String,
}

impl fmt::Display for RunResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "stdout: '«{}», stderr: «{}», error: «{}»",
            self.stdout, self.stderr, self.error
        )
    }
}

fn print_success(language: Language, run_result: RunResult) {
    println!(
        "[{}] {}: {}",
        green_text("SUCCESS"),
        language.config().id(),
        run_result.stdout.trim_end()
    );
}

fn print_failure(language: Language, error: Error) {
    println!(
        "[{}] {} ({}) - {}",
        red_text("FAILURE"),
        language.config().id(),
        language.config().run_config().container_image,
        error
    );
}

fn prepare_hello_request(language: Language) -> RunRequest {
    let config = language.config();
    let editor_config = config.editor_config();

    let files = vec![RequestFile {
        name: editor_config.default_filename.clone(),
        content: editor_config.example_code,
    }];

    let main_file = PathBuf::from(editor_config.default_filename);

    RunRequest {
        run_instructions: config.run_instructions(main_file, vec![]),
        files,
        stdin: None,
    }
}

fn prepare_version_request(language: Language) -> RunRequest {
    let config = language.config();

    RunRequest {
        run_instructions: RunInstructions {
            build_commands: vec![],
            run_command: config.run_config().version_command,
        },
        files: vec![],
        stdin: None,
    }
}

fn check_hello_result(run_result: &RunResult) -> Result<(), Error> {
    if !run_result.error.is_empty() {
        return Err(Error::RunResultErr(run_result.clone()));
    }

    check_hello_stderr(run_result)?;
    check_hello_stdout(&run_result.stdout)?;

    Ok(())
}

fn check_hello_stderr(run_result: &RunResult) -> Result<(), Error> {
    let expected_errors = [
        "Compiled in DEV mode. Follow the advice at https://elm-lang.org/0.19.1/optimize for better performance and smaller assets.\n"
    ];

    if run_result.stderr.is_empty() || expected_errors.contains(&run_result.stderr.as_str()) {
        Ok(())
    } else {
        Err(Error::RunResultStderr(run_result.clone()))
    }
}

fn check_hello_stdout(stdout: &str) -> Result<(), Error> {
    let normalized_text = stdout.trim_end().replace('"', "").to_lowercase();

    if normalized_text == "hello world!" {
        Ok(())
    } else {
        Err(Error::NoHelloWorld(stdout.to_string()))
    }
}

fn check_version_result(run_result: &RunResult) -> Result<(), Error> {
    if !run_result.error.is_empty() {
        return Err(Error::RunResultErr(run_result.clone()));
    }

    check_version_stderr(run_result)?;
    check_version_stdout(&run_result.stdout)?;

    Ok(())
}

fn check_version_stderr(run_result: &RunResult) -> Result<(), Error> {
    if run_result.stderr.is_empty() {
        Ok(())
    } else {
        Err(Error::RunResultStderr(run_result.clone()))
    }
}

fn check_version_stdout(stdout: &str) -> Result<(), Error> {
    if stdout.is_empty() {
        Err(Error::EmptyVersion)
    } else if stdout.trim_end().contains('\n') {
        Err(Error::VersionHasLineFeed(stdout.to_string()))
    } else {
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunRequest {
    run_instructions: RunInstructions,
    files: Vec<RequestFile>,
    stdin: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestFile {
    name: String,
    content: String,
}

pub struct Options {
    pub command: String,
    pub stdin: String,
}

pub fn run(options: Options) -> Result<SuccessOutput, Error> {
    let output = execute(options).map_err(Error::Execute)?;
    get_output(output).map_err(Error::Output)
}

pub enum Error {
    SerializeRequest(serde_json::Error),
    Execute(ExecuteError),
    Output(OutputError),
    DeserializeResult(serde_json::Error),
    NonEmptyStderr(String),
    RunResultErr(RunResult),
    RunResultStderr(RunResult),
    NoHelloWorld(String),
    EmptyVersion,
    VersionHasLineFeed(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SerializeRequest(err) => {
                write!(f, "Failed to serialize run request. {}", err)
            }

            Error::Execute(err) => {
                write!(f, "Error while executing command. {}", err)
            }

            Error::Output(err) => {
                write!(f, "Error in output from command. {}", err)
            }

            Error::DeserializeResult(err) => {
                write!(f, "Failed to deserialize run result. {}", err)
            }

            Error::NonEmptyStderr(err) => {
                write!(f, "Non-empty stderr. {}", err)
            }

            Error::RunResultErr(err) => {
                write!(f, "Error in run result. {}", err)
            }

            Error::RunResultStderr(err) => {
                write!(f, "Non-empty stderr in run result. {}", err)
            }

            Error::NoHelloWorld(err) => {
                write!(f, "No 'Hello World!' in stdout. {}", err)
            }

            Error::EmptyVersion => {
                write!(f, "Stdout is empty for version command.")
            }

            Error::VersionHasLineFeed(err) => {
                write!(f, "Version has line feed. {}", err)
            }
        }
    }
}

#[derive(Debug)]
pub enum ExecuteError {
    Execute(io::Error),
    CaptureStdin(),
    WriteStdin(io::Error),
    WaitForChild(io::Error),
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecuteError::Execute(err) => {
                write!(f, "{}", err)
            }

            ExecuteError::CaptureStdin() => {
                write!(f, "Failed to capture stdin.")
            }

            ExecuteError::WriteStdin(err) => {
                write!(f, "Failed to write to stdin. {}", err)
            }

            ExecuteError::WaitForChild(err) => {
                write!(f, "Failed while waiting for child. {}", err)
            }
        }
    }
}

pub fn execute(options: Options) -> Result<process::Output, ExecuteError> {
    let mut child = process::Command::new("sh")
        .arg("-c")
        .arg(options.command)
        .current_dir(".")
        .stdin(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .map_err(ExecuteError::Execute)?;

    child
        .stdin
        .as_mut()
        .ok_or(ExecuteError::CaptureStdin())?
        .write_all(options.stdin.as_bytes())
        .map_err(ExecuteError::WriteStdin)?;

    child.wait_with_output().map_err(ExecuteError::WaitForChild)
}

#[derive(Debug)]
pub struct SuccessOutput {
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub struct ErrorOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl fmt::Display for ErrorOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut messages = Vec::new();

        if let Some(code) = self.exit_code {
            messages.push(format!("code: {}", code));
        }

        if !self.stdout.is_empty() {
            messages.push(format!("stdout: {}", self.stdout))
        }

        if !self.stderr.is_empty() {
            messages.push(format!("stderr: {}", self.stderr))
        }

        write!(f, "{}", messages.join(", "))
    }
}

#[derive(Debug)]
pub enum OutputError {
    ExitFailure(ErrorOutput),
    ReadStdout(string::FromUtf8Error),
    ReadStderr(string::FromUtf8Error),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputError::ExitFailure(err) => {
                write!(f, "Exited with non-zero exit code. {}", err)
            }

            OutputError::ReadStdout(err) => {
                write!(f, "Failed to read stdout. {}", err)
            }

            OutputError::ReadStderr(err) => {
                write!(f, "Failed to read stderr. {}", err)
            }
        }
    }
}

pub fn get_output(output: process::Output) -> Result<SuccessOutput, OutputError> {
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).map_err(OutputError::ReadStdout)?;

        let stderr = String::from_utf8(output.stderr).map_err(OutputError::ReadStderr)?;

        Ok(SuccessOutput { stdout, stderr })
    } else {
        let stdout = String::from_utf8(output.stdout).map_err(OutputError::ReadStdout)?;

        let stderr = String::from_utf8(output.stderr).map_err(OutputError::ReadStderr)?;

        let exit_code = output.status.code();

        Err(OutputError::ExitFailure(ErrorOutput {
            stdout,
            stderr,
            exit_code,
        }))
    }
}

fn green_text(s: &str) -> String {
    format!("\x1b[92m{}\x1b[0m", s)
}

fn red_text(s: &str) -> String {
    format!("\x1b[91m{}\x1b[0m", s)
}
