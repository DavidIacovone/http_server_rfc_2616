use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    time_window: Duration,
}

impl RateLimiter {
    // Create a new rate limiter with a max request limit and a time window
    pub fn new(max_requests: usize, time_window: Duration) -> Self {
        RateLimiter {
            requests: HashMap::new(),
            max_requests,
            time_window,
        }
    }

    // Check if the given IP has exceeded the rate limit
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
