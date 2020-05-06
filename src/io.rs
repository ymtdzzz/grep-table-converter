use std::io::{Read, BufRead};
use std::fs::File;
use chrono::Utc;
use anyhow::Context as _;

use super::generator::Mode;

pub fn read_from_stdin<R>(mut reader: R) -> anyhow::Result<String>
where
    R: BufRead
{
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn read_from_file(filename: &String) -> anyhow::Result<String> {
    // TODO: make testable
    let mut buf = String::new();
    let mut f = File::open(filename)
        .with_context(|| format!("Could not open file: {}", filename))?;

    f.read_to_string(&mut buf)
     .with_context(|| format!("Something went wrong reading file: {}", filename))?;

    Ok(buf)
}

pub fn generate_filename(mode: &Mode) -> String {
    let formatted_date = Utc::now().format("%Y%m%d%H%M%S").to_string();
    String::from("output_") + formatted_date.as_str() + mode.extension()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stdin_read_test() {
        let input = b"this is a test";
        let result = read_from_stdin(&input[..]);
        assert_eq!(input.iter().map(|&s| s as char).collect::<String>(), result.unwrap());
    }

    #[test]
    fn test_generating_filenmae() {
        let formatted_date = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let mode = Mode::CSV;
        assert_eq!(
            String::from("output_") + formatted_date.as_str() + ".csv",
            generate_filename(&mode)
        );
    }
}
