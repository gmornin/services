pub fn mime_collapse(original: &str) -> &str {
    match original {
        "application/x-java-archive" => "application/zip",
        _ => original,
    }
}
