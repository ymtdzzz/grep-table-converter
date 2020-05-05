pub fn generate_csv(content: &str) -> &str {
    content
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let content = "test.rs:155:this is test.";
        let expect = "\
file_name,line_num,content
test.rs,155,this is test.";
        assert_eq!(expect, generate_csv(&content));
    }
}
