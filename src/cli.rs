use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::result::Result;

pub async fn start_cli() -> Result<(), Box<dyn std::error::Error>> {
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

        let uuid;
        match cache.get(name) {
            Some(val) => uuid = Some(val.to_string()),
            None => {
                let temp = crate::get_uuid_for_name(name).await;

                if temp.is_ok() {
                    uuid = Some(temp.unwrap())
                } else {
                    uuid = None;
                };
            }
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
    Ok(())
}
