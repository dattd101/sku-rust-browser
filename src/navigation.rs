use url::{form_urlencoded, Url};

pub const HOME_URL: &str = "https://www.google.com/";

pub fn resolve(input: &str) -> String {
    let input = input.trim();
    if input.is_empty() {
        return HOME_URL.to_owned();
    }

    if let Ok(url) = Url::parse(input) {
        if matches!(url.scheme(), "http" | "https") {
            return url.to_string();
        }
    }

    if !input.chars().any(char::is_whitespace)
        && (input.contains('.') || input.eq_ignore_ascii_case("localhost"))
    {
        let candidate = format!("https://{input}");
        if let Ok(url) = Url::parse(&candidate) {
            if url.host_str().is_some() {
                return url.to_string();
            }
        }
    }

    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("q", input)
        .finish();
    format!("https://www.google.com/search?{query}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domains() {
        assert_eq!(resolve("example.com"), "https://example.com/");
        assert_eq!(resolve("https://example.com/a"), "https://example.com/a");
    }

    #[test]
    fn searches() {
        assert_eq!(resolve("rust language"), "https://www.google.com/search?q=rust+language");
        assert_eq!(resolve(""), HOME_URL);
    }
}
