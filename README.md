# aisstream-rs

A Rust client for streaming real-time AIS vessel data from [aisstream.io](https://aisstream.io).

Connect over WebSocket, subscribe to a geographic area, and receive strongly typed messages on an async stream. The client sends your subscription automatically and reconnects when the connection drops.

[API documentation](https://aisstream.io/documentation) · [docs.rs](https://docs.rs/aisstream-rs)

## Installation

```toml
[dependencies]
aisstream-rs = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
futures-util = "0.3"
anyhow = "1"
```

## Quick start

You need an API key from [aisstream.io](https://aisstream.io).

```rust
use anyhow::Result;
use aisstream_rs::{AisStreamConfig, AisStreamCredentials, AisWebsocketClient};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AisStreamConfig::new(
        "wss://stream.aisstream.io".into(),
        vec![
            vec![[25.835302, -80.207729], [25.602700, -79.879297]],
            vec![[33.772292, -118.356139], [33.673490, -118.095731]],
        ],
        AisStreamCredentials {
            api_key: std::env::var("AISSTREAM_API_KEY")?,
            api_secret: None,
        },
        None,
        None,
    );

    let mut client = AisWebsocketClient::new(config);
    let mut stream = client.stream().await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("{message:#?}"),
            Err(err) => eprintln!("failed to parse message: {err}"),
        }
    }

    Ok(())
}
```

## Configuration

Config can be built in code or loaded from a JSON file:

```rust
use aisstream_rs::AisStreamConfig;
use std::path::Path;

let config = AisStreamConfig::from_file(Path::new("config.json"))?;
```

Example `config.json`:

```json
{
  "base_url": "wss://stream.aisstream.io",
  "bounding_boxes": [
    [[25.835302, -80.207729], [25.602700, -79.879297]]
  ],
  "credentials": {
    "api_key": "your-api-key"
  },
  "filter_ship_mmsi": ["368207620"],
  "filter_message_type": ["PositionReport"]
}
```

`base_url` should be the host only (`wss://stream.aisstream.io`). The client appends `/v0/stream`.

Bounding boxes are pairs of `[latitude, longitude]` corners:

```
[[[lat1, lon1], [lat2, lon2]], ...]
```

## Reconnection

The client reconnects automatically when the connection drops. By default it uses exponential backoff (1s → 2s → 4s → … up to 30s).

```rust
use aisstream_rs::AisWebsocketClient;

let client = AisWebsocketClient::new(config);
```

Provide your own strategy by implementing `AISStreamReconnectStrategy`:

```rust
use std::time::Duration;
use aisstream_rs::{AISStreamReconnectStrategy, AisWebsocketClient, ExponentialBackoff};

// Or tune the built-in backoff:
let strategy = ExponentialBackoff::new(Duration::from_secs(2), Duration::from_secs(60));
let client = AisWebsocketClient::with_strategy(config, strategy);
```

The message stream stays open across reconnects — you keep reading from the same `stream` handle.

## Logging

The library emits structured logs via [`tracing`](https://docs.rs/tracing). Wire up a subscriber in your application:

```rust
tracing_subscriber::fmt::init();
```

- `info` — connects, disconnects, reconnect delays
- `debug` — subscription details, config loading
- `trace` — individual AIS messages (can be very noisy at high throughput)
- `warn` / `error` — connection and parse failures

```toml
tracing-subscriber = "0.3"
```

## Message types

Each event is an [`AisMessage`](https://docs.rs/aisstream-rs/latest/aisstream-rs/struct.AisMessage.html) with:

- `message_type` — e.g. `PositionReport`
- `metadata` — ship name, MMSI, last known position
- `message` — the typed AIS payload as an [`AisStreamMessage`](https://docs.rs/aisstream-rs/latest/aisstream-rs/enum.AisStreamMessage.html) enum

```rust
use aisstream_rs::AisStreamMessage;

if let AisStreamMessage::PositionReport(report) = message.message {
    println!("MMSI {} at {}, {}", report.user_id, report.latitude, report.longitude);
}
```

## License

MIT — see [LICENSE](LICENSE).
