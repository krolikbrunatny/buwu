use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use extractor::*;
use reqwest::{blocking::Client, header};
use scraper::Html;
use serde::Deserialize;

mod extractor;

#[derive(Debug, PartialEq, Clone)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub return_date: NaiveDate,
}

#[derive(Debug, PartialEq)]
pub struct ProlongForm {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub username: String,
    pub password: String,
    pub days_threshold: u64,
}

impl AppConfig {
    pub fn get_credentials(&self) -> (&String, &String) {
        (&self.username, &self.password)
    }
}

pub fn build_http_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:124.0) Gecko/20100101 Firefox/124.0",
        ),
    );
    let client = Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()?;

    return Ok(client);
}

pub fn login(client: &Client, (username, password): (&String, &String)) -> Result<()> {
    let response = client
        .post("https://chamo.buw.uw.edu.pl:8443/auth/login")
        .body(format!(
            "username={}&password={}&forcelogin=1&login=Zaloguj",
            username, password
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()?
        .text()?;

    if let Some(_) = response.find("Wyloguj") {
        return Ok(());
    }

    match response.find("Nieprawidłowa nazwa użytkownika") {
        Some(_) => bail!("Invalid credentials."),
        _ => bail!("Cannot login. Unknown error."),
    }
}

pub fn prolong(client: &Client, prolong_form: &ProlongForm, books: &[&Book]) -> Result<usize> {
    if books.is_empty() {
        return Ok(0);
    }

    // Prepare body
    let mut body = prolong_form.id.clone() + "=";
    body.push_str(
        "&renewalCheckboxGroup%3AcheckoutsTable%3AtopToolbars%3Atoolbars%3A1%3Aspan%3ApageSize%3AsizeChoice=4",
    );
    body.push_str(
        "&renewalCheckboxGroup%3AcheckoutsTable%3AbottomToolbars%3Atoolbars%3A2%3Aspan%3ApageSize%3AsizeChoice=4",
    );
    for book in books {
        body.push_str(&format!("&renewalCheckboxGroup={}", book.id))
    }

    // Prepare url
    let url = format!(
        "https://chamo.buw.uw.edu.pl:8443/wicket/{}",
        prolong_form
            .url
            .strip_prefix("../")
            .context("Cannot format prolong form url.")?
    );

    let response = client
        .post(url)
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()?
        .text()?;

    let document = Html::parse_document(&response);
    let n = extract_prolonged_books_n(&document)?;

    if n != books.len() {
        bail!("Something went wrong. Books to prolong does not match prolonged books.");
    }

    Ok(n)
}

pub fn is_logged_in(client: &Client) -> Result<bool> {
    let response = client
        .get("https://chamo.buw.uw.edu.pl:8443/wicket/bookmarkable/com.vtls.chamo.webapp.component.patron.PatronAccountPage?theme=system")
        .send()?
        .text()?;

    match response.find("Wyloguj") {
        Some(_) => return Ok(true),
        _ => return Ok(false),
    }
}

pub fn scrape_books(client: &Client) -> Result<(ProlongForm, Vec<Book>)> {
    let response = client
        .get("https://chamo.buw.uw.edu.pl/wicket/bookmarkable/com.vtls.chamo.webapp.component.patron.PatronAccountPage?theme=system")
        .send()?
        .text()?;

    let document = Html::parse_document(&response);

    let prolong_form = extract_form(&document)?;
    let books = extract_books(&document)?;

    Ok((prolong_form, books))
}
