# ha-rs

A Rust library and command-line interface (CLI) tool for interacting with Home Assistant.

## Features

- 🦀 Pure Rust implementation with async support
- 🔌 Get entity states from Home Assistant
- 🎛️ Control devices (turn on/off)
- 🛠️ Simple and ergonomic API
- 📦 Use as a library or CLI tool

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
ha-rs = "0.1.0"
tokio = { version = "1.48", features = ["full"] }
```

### As a CLI Tool

```bash
cargo install --path . --bin ha_cli
```

Or build from source:

```bash
cargo build --release
./target/release/ha_cli --help
```

## Usage

### As a Library

```rust
use ha_rs::client::HaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let ha_url = "http://homeassistant.local:8123";
    let ha_token = "your_long_lived_access_token";
    let client = HaClient::new(ha_url.to_string(), ha_token.to_string());

    // Get entity state
    let entity = client.get_state("switch.smart_plug").await?;
    println!("Status: {}", entity.state);
    println!("Entity ID: {}", entity.entity_id);
    println!("Last updated: {}", entity.last_updated);

    // Turn on a device
    client.set_state("switch.smart_plug", true).await?;
    println!("Device turned on");

    // Turn off a device
    client.set_state("switch.smart_plug", false).await?;
    println!("Device turned off");

    Ok(())
}
```

### As a CLI Tool

Set required environment variables:

```bash
export HA_URL="http://homeassistant.local:8123"
export HA_TOKEN="your_long_lived_access_token"
```

Check device status:

```bash
ha_cli status
```

Toggle device state:

```bash
ha_cli toggle
```

## API Reference

### `HaClient`

The main client for interacting with Home Assistant.

#### Methods

##### `new(base_url: String, token: String) -> Self`

Creates a new Home Assistant client.

**Parameters:**
- `base_url`: The base URL of your Home Assistant instance (e.g., "http://homeassistant.local:8123")
- `token`: Your Home Assistant long-lived access token

**Example:**
```rust
let client = HaClient::new(
    "http://homeassistant.local:8123".to_string(),
    "your_token".to_string()
);
```

##### `async get_state(&self, entity_id: &str) -> Result<Entity, Error>`

Retrieves the current state of an entity.

**Parameters:**
- `entity_id`: The entity ID (e.g., "switch.smart_plug", "light.bedroom")

**Returns:**
- `Ok(Entity)`: Entity information including state, attributes, and timestamps
- `Err(Error)`: Network or API error

**Example:**
```rust
let entity = client.get_state("switch.smart_plug").await?;
println!("State: {}", entity.state);
```

##### `async set_state(&self, entity_id: &str, turn_on: bool) -> Result<String, Error>`

Controls a device by turning it on or off.

**Parameters:**
- `entity_id`: The entity ID (e.g., "switch.smart_plug", "light.bedroom")
- `turn_on`: `true` to turn on, `false` to turn off

**Returns:**
- `Ok(String)`: Response from Home Assistant API
- `Err(Error)`: Network or API error

**Example:**
```rust
// Turn on
client.set_state("switch.smart_plug", true).await?;

// Turn off
client.set_state("switch.smart_plug", false).await?;
```

### `Entity`

Represents a Home Assistant entity with its current state and metadata.

**Fields:**
- `entity_id: String` - The entity identifier
- `state: String` - Current state (e.g., "on", "off", "unavailable")
- `attributes: HashMap<String, serde_json::Value>` - Entity attributes
- `last_changed: String` - Timestamp of last state change
- `last_reported: String` - Timestamp of last report
- `last_updated: String` - Timestamp of last update
- `context: Context` - Context information

## Getting a Home Assistant Token

1. Log in to your Home Assistant instance
2. Click on your profile (bottom left)
3. Scroll down to "Long-Lived Access Tokens"
4. Click "Create Token"
5. Give it a name and copy the token

## Requirements

- Rust 2024 edition or later
- Home Assistant instance with API access
- Valid long-lived access token

## License

MIT

## Author

Sachin Kumar <sachinkumar.ar97@gmail.com>

## Repository

https://github.com/sachinkum0009/ha-rs

