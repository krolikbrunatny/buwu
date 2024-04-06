use std::{env, process};

use anyhow::Result;
use chrono::{Days, Local};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use log::*;
use simplelog::*;

use buwu::*;

fn setup_logger() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Never,
    )
    .unwrap();
}

fn load_config() -> Result<AppConfig> {
    let config: AppConfig = Figment::new()
        .merge(Toml::file(".config.toml"))
        .merge(Env::prefixed("DMB_"))
        .extract()?;

    Ok(config)
}

fn run_test_mode_only() -> Result<()> {
    let config = load_config()?;
    let client = build_http_client()?;
    match login(&client, config.get_credentials()) {
        Ok(_) => Ok(()),
        Err(_) => process::exit(1),
    }
}

fn main() {
    match try_main() {
        Ok(_) => (),
        Err(error) => error!("{}", error.to_string()),
    }
}

fn try_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let test_mode = match args.get(1) {
        Some(arg) => arg == "--test",
        None => false,
    };
    if test_mode {
        return run_test_mode_only();
    }

    setup_logger();

    info!("Loading config");
    let config = load_config()?;

    let client = build_http_client()?;

    info!("Logging in");
    login(&client, config.get_credentials())?;

    info!("Scrapping books");
    let (prolong_form, books) = scrape_books(&client)?;
    let filtered_books = filter_books(&books, config.days_threshold);

    match prolong(&client, &prolong_form, &filtered_books)? {
        0 => warn!("There are no books to prolong yet"),
        n => info!("The return date has been extended for {} books.", n),
    }

    info!("Done");

    Ok(())
}

fn filter_books(books: &Vec<Book>, days_threshold: u64) -> Vec<&Book> {
    let tomorrow = Local::now()
        .date_naive()
        .checked_add_days(Days::new(days_threshold))
        .unwrap();

    return books
        .iter()
        .filter(|b| tomorrow >= b.return_date)
        .collect::<Vec<&Book>>();
}
