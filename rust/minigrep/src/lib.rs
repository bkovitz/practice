pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        assert_eq!(
            vec!["safe, fast, productive."],
            search(
                "duct",
                "\
Rust:
safe, fast, productive.
Pick three.")
        );
    }
}