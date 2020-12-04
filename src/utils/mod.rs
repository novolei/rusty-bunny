pub mod github;
pub mod google;
pub mod twitter;

pub fn get_cmd_from_query(query: &str) -> &str {
    if query.contains(' ') {
        let index_of_whitespace = query.find(' ').unwrap_or(0);
        print!("query result is {}", &query);
        return &query[..index_of_whitespace];
    }
    &query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cmd_from_query_no_whitespace() {
        let actual = get_cmd_from_query("tw");
        let expacted = "tw";
        assert_eq!(actual, expacted);
    }
    #[test]
    fn test_get_cmd_from_query_with_whitespace() {
        let actual = get_cmd_from_query("tw @fbOpenSource");
        let expacted = "tw";
        assert_eq!(actual, expacted);
    }
}
