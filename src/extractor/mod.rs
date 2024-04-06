use anyhow::{Context, Result};
use chrono::NaiveDate;
use scraper::{Html, Selector};

use crate::{Book, ProlongForm};

#[cfg(test)]
mod tests;

pub fn extract_books(document: &Html) -> Result<Vec<Book>> {
    let mut books: Vec<Book> = Vec::new();

    let row_selector = Selector::parse("table#checkout tbody tr").unwrap();
    let col_selector = Selector::parse("td").unwrap();

    for row in document.select(&row_selector) {
        let cols = row.select(&col_selector).collect::<Vec<_>>();

        // Skip hidden rows with no columns
        if cols.len() < 8 {
            continue;
        }

        // id
        let id = cols.get(0).context("Cannot query id col.")?;
        // Skip books
        if let Some(_) = id.inner_html().find("Za wcześnie") {
            continue;
        }

        let id = id
            .select(&Selector::parse("input").unwrap())
            .next()
            .context("Cannot query checkbox input.")?
            .value()
            .attr("value")
            .context("Cannot get value attr of checkbox input.")?;

        // name
        let name = cols
            .get(1)
            .context("Cannot query name col.")?
            .text()
            .collect::<Vec<&str>>()
            .join(" ");
        let name = name.trim();

        // return date
        let return_date = cols
            .get(3)
            .context("Cannot query return date col.")?
            .text()
            .collect::<Vec<_>>()
            .join(" ");
        let return_date = return_date.trim();
        let return_date = NaiveDate::parse_from_str(return_date, "%Y-%m-%d %H:%M")?;

        let book = Book {
            id: id.to_string(),
            name: name.to_string(),
            return_date,
        };

        books.push(book);
    }

    return Ok(books);
}

pub fn extract_form(document: &Html) -> Result<ProlongForm> {
    // Form id
    let form_id_input_selector = Selector::parse("div#tabContents-3 > form > div > input").unwrap();
    let form_selector = Selector::parse("div#tabContents-3 > form").unwrap();
    return Ok(ProlongForm {
        id: document
            .select(&form_id_input_selector)
            .next()
            .context("Cannot query form id input.")?
            .value()
            .attr("id")
            .context("Cannot get attr id of form id input.")?
            .to_owned(),
        url: document
            .select(&form_selector)
            .next()
            .context("Cannot query prolong form.")?
            .value()
            .attr("action")
            .context("Cannot get attr action of prolong form.")?
            .to_owned(),
    });
}

pub fn extract_prolonged_books_n(document: &Html) -> Result<usize> {
    let success_msg_selector =
        Selector::parse("#main > div:nth-child(2) > h2:nth-child(1)").unwrap();

    return Ok(document
        .select(&success_msg_selector)
        .next()
        .context("Cannot query success message.")?
        .text()
        .next()
        .context("No text in success message.")?
        .split(" ")
        .next()
        .context("No results after splitting success message by whitespace.")?
        .parse::<usize>()?);
}
