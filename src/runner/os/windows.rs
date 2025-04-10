use super::output::Output;
use crate::Result;

pub fn execute(cmd: &str, stdin_data: &str) -> Result<(bool, Output)> {
    not_implemented!("Windows support is not implemented yet");
}
