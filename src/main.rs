pub mod jellyfin;
pub use crate::jellyfin::*;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use colored::Colorize;
use clap::Parser;
use retry::retry_with_index;

struct Config {
    rpc_client_id: String,
    url: String,
    api_key: String,
    username: String,
    enable_images: bool,
}

#[derive(Debug)]
enum ConfigError {
    MissingConfig,
    Io(std::io::Error),
    Var(std::env::VarError),
}

impl From<std::env::VarError> for ConfigError {
    fn from(value: std::env::VarError) -> Self {
        Self::Var(value)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Parser, Debug)]
#[command(author = "Xenon Colt")]
#[command(version)]
#[command(about = "Rich presence for jellyflix", long_about = None)]
struct Args {
    #[arg(short = 'c', long = "config", help = "Path to the config file")]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    dotenv::from_path(
        args.config.unwrap_or_else(|| 
            std::env::current_exe().unwrap()
            .parent().unwrap()
            .join(".env").to_string_lossy().to_string()
        )
    ).ok();
    let config = load_config().expect("Please make a file called .env and populate it with the needed variables");

    println!("{}\n                          {}", "//////////////////////////////////////////////////////////////////".bold(), "Jellyfin-RPC".bright_blue());

    if config.enable_images {
        println!("{}\n{}", "------------------------------------------------------------------".bold(), "Images won't work unless the server is forwarded!!!!".bold().red())
    }

    let mut connected: bool = false;
    let mut rich_presence_client = DiscordIpcClient::new(config.rpc_client_id.as_str()).expect("Failed to create Discord RPC client, discord is down or the Client ID is invalid.");

    // Start up the client connection, so that we can actually send and receive stuff
    connect(&mut rich_presence_client);
    println!("{}\n{}", "Connected to Discord Rich Presence Socket".bright_green().bold(), "------------------------------------------------------------------".bold());

    // Start loop
    loop {
        let content = get_jellyfin_playing(&config.url, &config.api_key, &config.username, &config.enable_images).await.unwrap();

        if !content.media_type.is_empty() {
            // Print what we're watching
            if !connected {
                println!("{}\n{}", content.details.bright_cyan().bold(), content.state_message.bright_cyan().bold());

                // Set connected to true so that we don't try to connect again
                connected = true;
            }

            // Set the activity
            let mut rpcbuttons: Vec<activity::Button> = std::vec::Vec::new();
            for i in 0..content.external_service_names.len() {
                rpcbuttons.push(activity::Button::new(
                    &content.external_service_names[i],
                    &content.external_service_urls[i],
                ));
            }

            rich_presence_client.set_activity(
                setactivity(&content.state_message, &content.details, content.endtime, &content.image_url, rpcbuttons)
            ).unwrap_or_else(|_| {
                rich_presence_client.reconnect().expect("Failed to reconnect");
            });

        } else if connected {
            // Disconnect from the client
            rich_presence_client.clear_activity().expect("Failed to clear activity");
            // Set connected to false so that we dont try to disconnect again
            connected = false;
            println!("{}\n{}\n{}", "------------------------------------------------------------------".bold(), "Cleared Rich Presence".bright_red().bold(), "------------------------------------------------------------------".bold());
        }
    // Sleep for 2 seconds
    std::thread::sleep(std::time::Duration::from_millis(750));
    }
}

fn load_config() -> Result<Config, Box<dyn core::fmt::Debug>> {
    let rpc_client_id = "1022477758556798986".to_string();
    let url = "https://stream.jellyflix.ga".to_string();
    let api_key = "".to_string();
    let username = dotenv::var("JELLYFIN_USERNAME").unwrap_or_else(|_| "".to_string());
    let enable_images = match dotenv::var("ENABLE_IMAGES").unwrap_or_else(|_| "".to_string()).to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        _ => false,
    };

    if username.is_empty() {
        return Err(Box::new(ConfigError::MissingConfig))
    }
    Ok(Config {
        rpc_client_id,
        url,
        api_key,
        username,
        enable_images,
    })
}

fn connect(rich_presence_client: &mut DiscordIpcClient) {
    println!("{}", "------------------------------------------------------------------".bold());
    retry_with_index(retry::delay::Exponential::from_millis(1000), |current_try| {
        println!("{} {}{}", "Attempt".bold().truecolor(225, 69, 0), current_try.to_string().bold().truecolor(225, 69, 0), ": Trying to connect".bold().truecolor(225, 69, 0));
        match rich_presence_client.connect() {
            Ok(result) => retry::OperationResult::Ok(result),
            Err(_) => {
                println!("{}", "Failed to connect, retrying soon".red().bold());
                retry::OperationResult::Retry(())
            },
        }
    }).unwrap();
}

fn setactivity<'a>(state_message: &'a String, details: &'a str, endtime: i64, image_url: &'a str, rpcbuttons: Vec<activity::Button<'a>>) -> activity::Activity<'a> {
    let mut new_activity = activity::Activity::new()
        .details(details)
        .timestamps(activity::Timestamps::new()
            .end(endtime)
        );
        

    if !image_url.is_empty() {
        new_activity = new_activity.clone().assets(
            activity::Assets::new()
                .large_image(image_url)
                .large_text(details)
                .small_image("https://xenoncolt.github.io/xenoncoltbot/jellyflix_512.jpg")
                .small_text("JellyFlix")
        )
    } else {
        new_activity = new_activity.clone().assets(
            activity::Assets::new()
                .large_image("https://xenoncolt.github.io/xenoncoltbot/jellyflix_512.jpg")
                .large_text("JellyFlix")
                .small_image("https://xenoncolt.github.io/xenoncoltbot/jellyflix_512.jpg")
                .small_text("JellyFlix")
        )
    }

    if !state_message.is_empty() {
        new_activity = new_activity.clone().state(state_message);
    }
    if !rpcbuttons.is_empty() {
        new_activity = new_activity.clone().buttons(rpcbuttons);
    }
    new_activity
}
