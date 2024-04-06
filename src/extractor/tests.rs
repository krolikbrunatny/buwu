use super::*;
use pretty_assertions::assert_eq;
use scraper::Html;

#[test]
fn test_extract_books_empty() {
    let expected: Vec<Book> = vec![];
    assert_eq!(extract_books(&Html::parse_fragment("")).unwrap(), expected);
}

#[test]
fn test_extract_books() {
    let expected = vec![
        Book {
            id: "check11".to_string(),
            name: "Od liczb zespolonych do kwadryk : zbiór zadań z algebry z rozwiązaniami / [oprac. i przyg. do druku Jacek Jezierski] ; Katedra Metod Matematycznych Fizyki. [Wydział Fizyki Uniwersytetu Warszawskiego].".to_string(),
            return_date: NaiveDate::from_ymd_opt(2024, 3, 4).unwrap(),
        },
        Book {
            id: "check12".to_string(),
            name: "Zbiór zadań z analizy matematycznej / Józef Banaś, Stanisław Wędrychowicz.".to_string(),
            return_date: NaiveDate::from_ymd_opt(2024, 3, 4).unwrap(),
        }
    ];

    let html = Html::parse_fragment(include_str!("html/books_1.html"));
    assert_eq!(extract_books(&html).unwrap(), expected);
}

#[test]
fn test_extract_form() {
    let html = Html::parse_fragment(include_str!("html/books_1.html"));
    let form = ProlongForm {
        id: "id34_hf_0".to_string(),
        url: "../page?4-1.IFormSubmitListener-patronAccount-tabs-tabContents-3-renewalForm"
            .to_string(),
    };

    assert_eq!(extract_form(&html).unwrap(), form);
}

#[test]
fn test_extract_prolonged_books_n() {
    let html = Html::parse_document(include_str!("html/success_page_1.html"));

    assert_eq!(extract_prolonged_books_n(&html).unwrap(), 1);
}
