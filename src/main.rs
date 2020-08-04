mod cli;
mod code;
mod pattern;

use code::Code;
use std::{
    fs,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let options = cli::run();
    let code_str =
        fs::read_to_string(options.0).expect("Cannot read the input file");
    let code_str = pattern::code_fmt(&code_str[..]);
    let code = Code::new(code_str).format();
    match options.1.len() {
        0 => {
            io::stdout().write_all(code.as_bytes())?;
        }
        _ => {
            fs::write(options.1, code)?;
        }
    }
    Ok(())
}
