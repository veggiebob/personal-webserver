use std::ffi::OsStr;
use std::io::Stdout;
use std::process::{Command, Stdio};

const EXECUTABLE_NAME: &str = "left-right-parser";
pub fn run_parse_demo<X: AsRef<OsStr>, Y: AsRef<OsStr>>(input: X, mode: Y) -> Result<String, String> {
    let response = Command::new(EXECUTABLE_NAME)
        .arg("e") // evaluation mode
        .arg(mode) // 'stmt', 'expr', or 'prgm'*
        .arg(input)
        .stdout(Stdio::piped())
        .output();
    // prgm mode not supported yet so don't pass it
    match response {
        Err(e) => Err(format!("An error occurred when trying to run the program. \
            Is {} in the PATH? {}", EXECUTABLE_NAME, e.to_string())),
        Ok(res) => Ok(
            std::str::from_utf8(&res.stdout[..]).unwrap().to_string()
        )
    }
}

#[test]
fn test_run_demo() {
    println!("{:?}", run_parse_demo("x + y", "expr"));
}