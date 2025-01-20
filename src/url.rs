use std::collections::HashMap;

pub fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if query.is_empty() {
        return params; // Return empty HashMap if no query params
    }

    // Split the query string by '&' to get individual key-value pairs
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.insert(key.to_string(), value.to_string());
        }
    }

    params
}
