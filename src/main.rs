pub mod jellyfin;
pub use crate::jellyfin::*;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use colored::Colorize;
use clap::Parser;
use retry::retry_with_index;

struct Config {
    url: String,
    api_key: String,
    username: String,
    rpc_client_id: String,
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
#[command(about = "Rich presence for JellyFlix", long_about = None)]
struct Args {
    #[arg(short = 'c', long = "config", help = "Path to the config file")]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = args.config.unwrap_or_else(||
        if cfg!(not(windows)) {
            if std::env::var("USER").unwrap() != *"root" {
                std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_|
                    {
                        let mut dir = std::env::var("HOME").unwrap();
                        dir.push_str("/.config/jellyflix-rpc/main.json");
                        dir
                    }
                )
            } else {
                "/etc/jellyflix-rpc/main.json".to_string()
            }
        } else {
            let mut dir = std::env::var("APPDATA").unwrap();
            dir.push_str("\\jellyflix-rpc\\main.json");
            dir
        }
    );

    std::fs::create_dir_all(std::path::Path::new(&config_path).parent().unwrap()).ok();

    if config_path.ends_with(".env") {
        panic!("\n{}\n(Example: https://github.com/Radiicall/jellyfin-rpc/blob/main/example.json)\n", "Please update your .env to JSON format.".bold().red())
    }

    let config = load_config(
        config_path.clone()
    ).unwrap_or_else(|_| panic!("\n\nPlease populate your config file '{}' with the needed variables\n(https://github.com/Radiicall/jellyfin-rpc#setup)\n\n", std::fs::canonicalize(config_path).unwrap().to_string_lossy()));

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
            rpcbuttons.push(activity::Button::new(
                "Watch Now",
                "https://stream.jellyflix.ga",
              ));

            rpcbuttons.push(activity::Button::new(
              "Website",
              "https://info.jellyflix.ga",
              ));

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

    std::thread::sleep(std::time::Duration::from_millis(750));
    }
}

fn load_config(path: String) -> Result<Config, Box<dyn core::fmt::Debug>> {
    let data = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("\n\nPlease make the file '{}' and populate it with the needed variables\n(https://github.com/Radiicall/jellyfin-rpc#setup)\n\n", path));
    let res: serde_json::Value = serde_json::from_str(&data).unwrap_or_else(|_| panic!("{}", "\nUnable to parse config file. Is this a json file?\n".red().bold()));

    let jellyfin: serde_json::Value = res["Jellyfin"].clone();
    let discord: serde_json::Value = res["Discord"].clone();

    let url = "https://stream.jellyflix.ga".to_string();
    let api_key = "0364822cdce64d149ab1d29376d51c29".to_string();
    let username = jellyfin["USERNAME"].as_str().unwrap().to_string();
    let rpc_client_id = "1022477758556798986".to_string();
    let enable_images = discord["ENABLE_IMAGES"].as_bool().unwrap_or_else(|| 
        panic!(
            "\n{}\n{} {} {} {}\n",
            "ENABLE_IMAGES has to be a bool...".red().bold(),
            "EXAMPLE:".bold(), "true".bright_green().bold(), "not".bold(), "'true'".red().bold()
        )
    );

    if rpc_client_id.is_empty() || url.is_empty() || api_key.is_empty() || username.is_empty() {
        return Err(Box::new(ConfigError::MissingConfig))
    }
    Ok(Config {
        url,
        api_key,
        username,
        rpc_client_id,
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

fn setactivity<'a>(state_message: &'a String, details: &'a str, endtime: Option<i64>, image_url: &'a str, rpcbuttons: Vec<activity::Button<'a>>) -> activity::Activity<'a> {
    let mut new_activity = activity::Activity::new()
        .state("Watching")
        .details(details);

    let mut assets = activity::Assets::new();


    match endtime {
        Some(time) => {
            new_activity = new_activity.clone().timestamps(activity::Timestamps::new()
                .end(time)
            );
            assets = assets.clone()
            .small_image("https://xenoncolt.github.io/xenoncoltbot/jellyflix_512.jpg")
            .small_text("JellyFlix");
        },
        None => {
            assets = assets.clone()
            .small_image("https://xenoncolt.github.io/file_storage/jellyflix-rpc/pause.png")
            .small_text("Paused");
        },
    }

    if !image_url.is_empty() {
        new_activity = new_activity.clone().assets(
            assets.clone()
                .large_image(image_url)
                .large_text(details)
        )
    } else {
        new_activity = new_activity.clone().assets(
            assets.clone()
                .large_image("https://xenoncolt.github.io/xenoncoltbot/jellyflix_512.jpg")
                .large_text("JellyFlix")
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
