use std::ffi::OsStr;
use std::process::{Command, Stdio};

const EXECUTABLE_NAME: &str = "gym-population";
pub fn query_gym_data<X: AsRef<OsStr>>(input: X) -> Result<String, String> {
    let response = Command::new(EXECUTABLE_NAME)
        .stdout(Stdio::piped())
        .output();
    match response {
        Err(e) => Err(format!("An error occurred when trying to run the program. \
            Is {} in the PATH? {}", EXECUTABLE_NAME, e.to_string())),
        Ok(res) => {
            let str_output = std::str::from_utf8(&res.stdout[..]);
            let s = str_output.unwrap().to_string();
            if s.trim().len() == 0 {
                Err(format!("The program was run successfully, but there was no output! Is there sufficient data?"))
            } else {
                Ok(s)
            }
        }
    }
}