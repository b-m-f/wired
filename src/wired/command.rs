use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn run(command: String, args: &[String]) -> Result<String, String> {
    let mut cmd = Command::new(command.clone());
    for arg in args {
        cmd.arg(arg);
    }
    match cmd.output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(string) => return Ok(string),
            Err(e) => return Err(format!("{e}")),
        },
        Err(e) => return Err(format!("Error when executing command {command}: {e}")),
    }
}
pub fn run_with_input_on_stdin(
    command: String,
    args: &[String],
    input: String,
) -> Result<String, String> {
    let mut cmd = Command::new(command.clone());
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    for arg in args {
        cmd.arg(arg);
    }
    let mut cmd = cmd.spawn().unwrap();
    let mut stdin = cmd.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Passing privatekey to wg pubkey");
    });

    let output = cmd
        .wait_with_output()
        .expect("Getting publickey for privatekey");
    let output_string: String = String::from_utf8(output.stdout).unwrap();
    return Ok(output_string.trim_end().to_string());
}
