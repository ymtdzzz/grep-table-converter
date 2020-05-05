pub fn generate_csv(content: &str) -> String {
    let mut result = String::from("file_name,line_num,content\n");
    for line in content.lines() {
        let splitted_line: Vec<&str> = line.split(":").collect();
        let file_path = &splitted_line[0];
        let line_num = &splitted_line[1];
        let remaining = &splitted_line[2..].join(":").to_string();

        result += format!("{},{},{}\n", file_path, line_num, remaining).as_str();
    }
    println!("{}", result);
    return result;
}

pub fn generate_markdown(content: &str) -> String {
    let mut result = r#"| file_name | line_num | content |
| --- | --- | --- |
"#.to_string();
    for line in content.lines() {
        let splitted_line: Vec<&str> = line.split(":").collect();
        let file_path = &splitted_line[0];
        let line_num = &splitted_line[1];
        let remaining = &splitted_line[2..].join(":").to_string();

        result += format!("| {} | {} | {} |\n", file_path, line_num, remaining).as_str();
    }
    println!("{}", result);
    return result;
}

pub fn generate_textile(content: &str) -> String {
    let mut result = r#"|file_name|line_num|content|
"#.to_string();
    for line in content.lines() {
        let splitted_line: Vec<&str> = line.split(":").collect();
        let file_path = &splitted_line[0];
        let line_num = &splitted_line[1];
        let remaining = &splitted_line[2..].join(":").to_string();

        result += format!("|{}|{}|{}|\n", file_path, line_num, remaining).as_str();
    }
    println!("{}", result);
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn csv_result() {
        let content = r#"test.rs:155:this is test
test.rs:201:TestCrate::test_method();
modules/hoge_module.rs:14:println!("this is test String.");"#;
        let expect = r#"file_name,line_num,content
test.rs,155,this is test
test.rs,201,TestCrate::test_method();
modules/hoge_module.rs,14,println!("this is test String.");
"#;
        assert_eq!(expect, generate_csv(&content));
    }

    #[test]
    fn markdown_result() {
        let content = r#"test.rs:155:this is test
test.rs:201:TestCrate::test_method();
modules/hoge_module.rs:14:println!("this is test String.");"#;
        let expect = r#"| file_name | line_num | content |
| --- | --- | --- |
| test.rs | 155 | this is test |
| test.rs | 201 | TestCrate::test_method(); |
| modules/hoge_module.rs | 14 | println!("this is test String."); |
"#;
        assert_eq!(expect, generate_markdown(&content));
    }

    #[test]
    fn textile_result() {
        let content = r#"test.rs:155:this is test
test.rs:201:TestCrate::test_method();
modules/hoge_module.rs:14:println!("this is test String.");"#;
        let expect = r#"|file_name|line_num|content|
|test.rs|155|this is test|
|test.rs|201|TestCrate::test_method();|
|modules/hoge_module.rs|14|println!("this is test String.");|
"#;
        assert_eq!(expect, generate_textile(&content));
    }
}
