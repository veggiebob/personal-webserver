use std::ffi::OsStr;
use std::io::{Stdout, Write};
use std::process::{Command, Stdio};

const EXECUTABLE_NAME: &str = "bf2spl";
pub fn run_translate_demo<X: ToString, Y: AsRef<OsStr>>(input: X, mode: Y) -> Result<String, String> {
    let mut p = Command::new(EXECUTABLE_NAME)
        .arg(mode) // '' or 'ai'
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    if let Some(mut stdin) = p.stdin.take() {
        stdin.write_all(input.to_string().as_bytes()).unwrap();
    }
    let response = p.wait_with_output();
    match response {
        Err(e) => Err(format!("An error occurred when trying to run the program. \
            Is {} in the PATH? {}", EXECUTABLE_NAME, e.to_string())),
        Ok(res) => Ok({
            let str_output = std::str::from_utf8(&res.stdout[..]);
            // println!("here is the output of the parse program with input {:?}: {:?}", input.as_ref().to_str(), &str_output);
            str_output.unwrap().to_string()
        })
    }
}

#[test]
fn test_run_demo() {
    println!("{:?}", run_translate_demo(",[.,]", ""));
}