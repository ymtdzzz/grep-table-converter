use std::io::{self, Read};
use std::fs::File;
use atty::Stream;
use chrono::Utc;
use anyhow::Context as _;

use super::generator::Mode;

pub fn read_from_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn read_from_file(filename: &String) -> anyhow::Result<String> {
    let mut buf = String::new();
    let mut f = File::open(filename)
        .with_context(|| format!("Could not open file: {}", filename))?;

    f.read_to_string(&mut buf)
     .with_context(|| format!("Something went wrong reading file: {}", filename))?;

    Ok(buf)
}

pub fn is_pipe() -> bool {
    ! atty::is(Stream::Stdin)
}

pub fn generate_filename(mode: &Mode) -> String {
    let formatted_date = Utc::now().format("%Y%m%d%H%M%S").to_string();
    String::from("output_") + formatted_date.as_str() + mode.extension()
}

#[cfg(test)]
mod test {
    use super::*;

}
