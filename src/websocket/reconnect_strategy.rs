//! Reconnect delay strategies used when the WebSocket connection drops.
//!
//! The default is [`ExponentialBackoff`]. Implement [`AISStreamReconnectStrategy`] on your own type
//! and pass it to [`AisWebsocketClient::with_strategy`](super::ais_websocket_client::AisWebsocketClient::with_strategy).

use std::time::Duration;

use tracing::debug;

/// Controls how long to wait before trying to reconnect after a disconnect.
pub trait AISStreamReconnectStrategy {
    /// Returns the delay before the next connection attempt.
    fn next_delay(&mut self) -> Duration;

    /// Called after a successful connection. Reset any internal attempt counter here.
    fn reset(&mut self);
}

/// Waits 1s, then 2s, 4s, 8s, … capping at 30s between attempts.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    attempt: u32,
    base: Duration,
    max: Duration,
}

impl ExponentialBackoff {
    /// Create a backoff strategy with custom base and maximum delays.
    pub fn new(base: Duration, max: Duration) -> Self {
        Self {
            attempt: 0,
            base,
            max,
        }
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            attempt: 0,
            base: Duration::from_secs(1),
            max: Duration::from_secs(30),
        }
    }
}

impl AISStreamReconnectStrategy for ExponentialBackoff {
    fn next_delay(&mut self) -> Duration {
        let factor = 2u64.pow(self.attempt);
        self.attempt += 1;

        let delay_ms = self.base.as_millis() as u64 * factor;
        let delay = Duration::from_millis(delay_ms.min(self.max.as_millis() as u64));
        debug!(
            attempt = self.attempt,
            delay_secs = delay.as_secs_f64(),
            "calculated exponential backoff delay"
        );
        delay
    }

    fn reset(&mut self) {
        debug!("reset reconnect backoff after successful connection");
        self.attempt = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_until_max() {
        let mut strategy = ExponentialBackoff::new(Duration::from_secs(1), Duration::from_secs(4));

        assert_eq!(strategy.next_delay(), Duration::from_secs(1));
        assert_eq!(strategy.next_delay(), Duration::from_secs(2));
        assert_eq!(strategy.next_delay(), Duration::from_secs(4));
        assert_eq!(strategy.next_delay(), Duration::from_secs(4));
    }

    #[test]
    fn reset_starts_over() {
        let mut strategy = ExponentialBackoff::default();
        let _ = strategy.next_delay();
        let _ = strategy.next_delay();
        strategy.reset();
        assert_eq!(strategy.next_delay(), Duration::from_secs(1));
    }
}
