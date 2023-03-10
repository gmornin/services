use mongodb::bson::Regex;

pub fn case_insensitive(query: String) -> Regex {
    Regex {
        pattern: query,
        options: String::from("i"),
    }
}
