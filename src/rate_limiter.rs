use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A struct representing a rate limiter.
///
/// This struct keeps track of the number of requests from different IP addresses
/// and enforces a maximum number of requests within a specified time window.
pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    time_window: Duration,
}

impl RateLimiter {

    /// Creates a new `RateLimiter`.
    ///
    /// # Arguments
    ///
    /// * `max_requests` - The maximum number of requests allowed within the time window.
    /// * `time_window` - The duration of the time window for rate limiting.
    ///
    /// # Returns
    ///
    /// A new `RateLimiter` instance.
    pub fn new(max_requests: usize, time_window: Duration) -> Self {
        RateLimiter {
            requests: HashMap::new(),
            max_requests,
            time_window,
        }
    }

    /// Checks if the given IP address is rate-limited.
    ///
    /// This function records the current request time, removes outdated requests,
    /// and checks if the number of requests from the given IP address exceeds the
    /// maximum allowed within the specified time window.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address of the client.
    ///
    /// # Returns
    ///
    /// * `true` if the IP address is rate-limited, `false` otherwise.
    pub fn is_rate_limited(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        let request_times = self.requests.entry(ip.to_string()).or_insert_with(Vec::new);

        // Remove outdated requests
        request_times.retain(|&time| now.duration_since(time) < self.time_window);

        // Check if the number of requests exceeds the limit
        if request_times.len() >= self.max_requests {
            return true;
        }

        // Add the current request time
        request_times.push(now);
        false
    }
}
