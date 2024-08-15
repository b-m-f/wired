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
    let mut cmd = match cmd.spawn() {
        Ok(cmd) => cmd,
        Err(e) => return Err(e.to_string()),
    };
    let mut stdin = match cmd.stdin.take() {
        Some(cmd) => cmd,
        None => {
            return Err(
                "Could not get access to sub-process '{command}' stdin. Aborting".to_string(),
            )
        }
    };

    let command_clone = command.clone();
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect(format!("Passing data into stdin of {command_clone}").as_str());
    });

    let output = cmd
        .wait_with_output()
        .expect(format!("Getting stdout of {command}").as_str());
    let output_string: String = String::from_utf8(output.stdout).unwrap();
    return Ok(output_string.trim_end().to_string());
}
pub fn encrypt_with_pass(destination: String, input: String) -> Result<String, String> {
    let mut cmd = Command::new("pass");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::null());
    for arg in ["insert", "-e", &destination] {
        cmd.arg(arg);
    }
    let mut cmd = match cmd.spawn() {
        Ok(cmd) => cmd,
        Err(e) => return Err(e.to_string()),
    };
    let mut stdin = match cmd.stdin.take() {
        Some(cmd) => cmd,
        None => {
            return Err("Could not get access to sub-process 'pass' stdin. Aborting".to_string())
        }
    };
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Passing data to pass");
    });

    let output = cmd
        .wait_with_output()
        .expect("Getting publickey for privatekey");
    let output_string: String = String::from_utf8(output.stdout).unwrap();
    return Ok(output_string.trim_end().to_string());
}
