mod code;
mod pattern;

use std::{
    env, fs,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filepath = args[1].clone();

    let content = fs::read_to_string(filepath).expect("Something went wrong reading the file");

    let code_fmt = pattern::code_fmt(&content);
    let mut code = code::Code::new(code_fmt);

    io::stdout().write_all(code.format().as_bytes())?;
    Ok(())
}
