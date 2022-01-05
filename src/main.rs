mod cli;
use clap::{arg, App};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MojangResponse {
    pub name: String,
    pub id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = App::new("Minecraft UUID fetcher")
        .author("Juubes")
        .about("Prints the player's UUID with the name provided.")
        .version("1.0.0")
        .arg(arg!(-p --player <player> "Player's IGN."))
        .arg(arg!(-I --interactive "Open command line."))
        .get_matches();

    if flags.is_present("interactive") {
        cli::start_cli().await?;
    } else {
        if !flags.is_present("player") {
            println!("Please use the --player <name> or the -I for interactive.");
            return Ok(());
        }
        let name = flags.value_of("player").unwrap();
        let uuid = get_uuid_for_name(name).await;

        if uuid.is_err() {
            println!("Error occurred.");
            return Ok(());
        }

        println!("UUID for {}: {}", name, uuid.unwrap());
    }
    Ok(())
}

pub async fn get_uuid_for_name(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get(format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        name
    ))
    .await?;

    let data = res.json::<MojangResponse>().await?;

    Ok(data.id)
}
