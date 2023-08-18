use mongodb::bson::Regex;

pub fn case_insensitive(query: String) -> Regex {
    Regex {
        pattern: format!("^{}$", regex::escape(&query)),
        options: String::from("i"),
    }
}
