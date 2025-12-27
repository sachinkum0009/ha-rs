/**
 * A bin to communicate with home assistant
 */

use std::env;
use std::io;

use ha_rs::client::HaClient;
use clap::{Parser, Subcommand, ValueEnum, CommandFactory};
use clap_complete::{generate, shells::{Bash, Zsh}};

#[derive(Parser)]
#[command(
    name = "ha_control",
    author,
    version,
    about = "Control Home Assistant",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show whether the device is On or Off
    Status,
    /// Toggle the device (turn on if off, turn off if on)
    Toggle,
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum Shell {
    Bash,
    Zsh,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle completions first (no HA connection needed)
    if let Commands::Completions { shell } = cli.command {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();

        match shell {
            Shell::Bash => generate(Bash, &mut cmd, bin_name, &mut io::stdout()),
            Shell::Zsh => generate(Zsh, &mut cmd, bin_name, &mut io::stdout()),
        }

        return Ok(());
    }

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
            if entity.state == "off" {
                client.set_state(entity_id, true).await?;
                println!("Turned ON.");
            } else {
                client.set_state(entity_id, false).await?;
                println!("Turned OFF.");
            }
        }
        // already handled above, this arm is just to satisfy exhaustiveness
        Commands::Completions { .. } => {}
    }

    Ok(())
}

