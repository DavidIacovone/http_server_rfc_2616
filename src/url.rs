use std::collections::HashMap;

/// Parses query parameters from a query string.
///
/// This function takes a query string (the part of a URL after the `?`) and
/// returns a `HashMap` where the keys and values are the query parameters.
///
/// # Arguments
///
/// * `query` - A string slice that holds the query part of a URL.
///
/// # Returns
///
/// A `HashMap` containing the query parameters as key-value pairs.
///
/// # Examples
///
/// ```
/// let query = "name=John&age=30";
/// let params = parse_query_params(query);
/// assert_eq!(params.get("name"), Some(&"John".to_string()));
/// assert_eq!(params.get("age"), Some(&"30".to_string()));
/// ```
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
