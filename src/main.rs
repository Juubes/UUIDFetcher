use clap::{arg, App};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, Write};
use std::option::Option;

#[derive(Deserialize, Debug)]
pub struct MojangResponse {
    pub name: String,
    pub id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Minecraft UUID fetcher")
        .author("Juubes")
        .about("Prints the player's UUID with the name provided.")
        .version("1.0.0")
        .arg(arg!(-p --player <player> "Player's IGN.").required(false))
        .arg(arg!(-I --interactive "Open command line."))
        .get_matches();

    if matches.is_present("interactive") {
        let mut cache: HashMap<String, String> = HashMap::new();

        println!("CLI ready! Insert a name to fetch.");

        print!("> ");
        io::stdout().flush()?;

        let mut buf: String = String::new();
        loop {
            buf.clear();
            if io::stdin().read_line(&mut buf).is_err() {
                break;
            }
            let name = buf.trim();
            if name.is_empty() {
                print!("> ");
                io::stdout().flush()?;
                continue;
            }

            let uuid: Option<String>;
            if cache.contains_key(name) {
                uuid = Some(cache.get(name).unwrap().to_string());
            } else {
                let temp = get_uuid_for_name(name).await;
                if temp.is_ok() {
                    uuid = Some(temp.unwrap())
                } else {
                    uuid = None;
                };
            }

            if uuid.is_none() {
                println!("Error occurred.");
            } else {
                let uuid = uuid.unwrap();
                cache.insert(name.to_owned(), uuid.clone());
                println!("UUID for {}: {}", name, uuid);
            }

            print!("> ");
            io::stdout().flush()?;
        }
    } else {
        if !matches.is_present("player") {
            println!("Please use the --player <name> or the -I for interactive.");
            return Ok(());
        }
        let name = matches.value_of("player").unwrap();
        let uuid = get_uuid_for_name(name).await;

        if uuid.is_err() {
            println!("Error occurred.");
            return Ok(());
        }

        println!("UUID for {}: {}", name, uuid.unwrap());
    }
    Ok(())
}

async fn get_uuid_for_name(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get(format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        name
    ))
    .await?;

    let data = res.json::<MojangResponse>().await?;

    Ok(data.id)
}
