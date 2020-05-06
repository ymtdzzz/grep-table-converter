use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
pub enum Mode {
    CSV,
    MARKDOWN,
    TEXTILE,
}

impl Mode {
    fn header(&self) -> String {
        match self {
            Mode::CSV => "file_name,line_num,content\n".to_string(),
            Mode::MARKDOWN => "| file_name | line_num | content |\n| --- | --- | --- |\n".to_string(),
            Mode::TEXTILE => "|file_name|line_num|content|\n".to_string(),
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    file_path: &'a str,
    line_num: &'a str,
    remaining: &'a str,
}

impl Line<'_> {
    fn format(&self, mode: &Mode) -> String {
        match mode {
            Mode::CSV => format!("{},{},{}\n", self.file_path, self.line_num, self.remaining),
            Mode::MARKDOWN => format!("| {} | {} | {} |\n", self.file_path, self.line_num, self.remaining),
            Mode::TEXTILE => format!("|{}|{}|{}|\n", self.file_path, self.line_num, self.remaining),
        }
    }
}

pub fn generate_table(content: &str, mode: &Mode) -> Result<String> {
    let mut result = String::from(mode.header());
    for line in content.lines() {
        let splitted_line: Vec<&str> = line.split(":").collect();

        if splitted_line.len() < 3 {
            return Err(anyhow!("Format of given line is invalid: {}", &line));
        }

        let line_data = Line {
            file_path: &splitted_line[0],
            line_num: &splitted_line[1],
            remaining: &splitted_line[2..].join(":").to_string(),
        };

        result += line_data.format(mode).as_str();
    }
    println!("{}", result);
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn csv_result() {
        let mode = Mode::CSV;
        let content = r#"test.rs:155:this is test
test.rs:201:TestCrate::test_method();
modules/hoge_module.rs:14:println!("this is test String.");"#;
        let expect = r#"file_name,line_num,content
test.rs,155,this is test
test.rs,201,TestCrate::test_method();
modules/hoge_module.rs,14,println!("this is test String.");
"#;
        assert_eq!(expect, generate_table(&content, &mode).unwrap());
    }

    #[test]
    fn markdown_result() {
        let mode = Mode::MARKDOWN;
        let content = r#"test.rs:155:this is test
test.rs:201:TestCrate::test_method();
modules/hoge_module.rs:14:println!("this is test String.");"#;
        let expect = r#"| file_name | line_num | content |
| --- | --- | --- |
| test.rs | 155 | this is test |
| test.rs | 201 | TestCrate::test_method(); |
| modules/hoge_module.rs | 14 | println!("this is test String."); |
"#;
        assert_eq!(expect, generate_table(&content, &mode).unwrap());
    }

    #[test]
    fn textile_result() {
        let mode = Mode::TEXTILE;
        let content = r#"test.rs:155:this is test
test.rs:201:TestCrate::test_method();
modules/hoge_module.rs:14:println!("this is test String.");"#;
        let expect = r#"|file_name|line_num|content|
|test.rs|155|this is test|
|test.rs|201|TestCrate::test_method();|
|modules/hoge_module.rs|14|println!("this is test String.");|
"#;
        assert_eq!(expect, generate_table(&content, &mode).unwrap());
    }

    #[test]
    fn invalid_format() {
        let mode = Mode::CSV;
        let content = "This is a test for invalid format error.";
        let expected_errmsg = format!("Format of given line is invalid: {}", &content);
        if let Err(e) = generate_table(&content, &mode) {
            assert_eq!(expected_errmsg, format!("{:?}", e));
        } else {
            assert!(false, "result should be error");
        }
    }
}
