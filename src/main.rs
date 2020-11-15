#[macro_use]
extern crate clap;
use clap::App;

use std::error::Error;

use gtrending;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let language = matches.value_of("language");
    let spoken_language = matches.value_of("spoken_language_code");
    let since = matches.value_of("since");
    //might be preferable using a tuple to send in arguments here.
    let results = gtrending::fetch_repos(language, spoken_language, since).await?;
    println!("{:?}", results.len());
    Ok(())
}
