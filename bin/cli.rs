/**
 * A bin to communicate with home assistant
 */

use std::env;
use ha_rs::client::HaClient;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ha_control", author, version, about = "Control Home Assistant", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show wheter the device is On or Off
    Status,
    /// Toggle the device (turn on if off, turn off if on)
    Toggle,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let ha_url = env::var("HA_URL").map_err(|_| "HA_URL env var is required")?;
    let ha_token = env::var("HA_TOKEN").map_err(|_| "HA_TOKEN env var is required")?;
    let client = HaClient::new(ha_url, ha_token);
    let entity_id = "switch.smart_plug";

    match cli.command {
        Commands::Status => {
            let entity = client.get_state(entity_id).await?;
            println!("Status: {}", entity.state);
        }
        Commands::Toggle => {
            let entity = client.get_state(entity_id).await?;
            // println!("Device is : {}", entity.state);
            if entity.state == "off" {
                client.set_state(entity_id, true).await?;
                println!("Turned ON.")
            }
            else {
                client.set_state(entity_id, false).await?;
                println!("Turned OFF.")
            }
        }
    }


    // // call the async method and handle the result
    // match client.get_state(entity_id).await {
    //     Ok(response) => {
    //         println!("Entity id: {}, state: {}", response.entity_id, response.state);
    //     }
    //     Err(e) => {
    //         eprintln!("Error getting state: {}", e);
    //     }
    // }

    // let result = client.set_state(entity_id, false).await;
    // match result {
    //     Ok(res) => {
    //         println!("Response: {}", res);
    //     }
    //     Err(e) => {
    //         eprintln!("Error setting state: {}", e);
    //     }
    // }
    Ok(())
}
