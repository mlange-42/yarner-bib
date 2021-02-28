use biblatex::{ChunksExt, Date, DateValue, Entry, Person};
use std::fmt::Write;

pub fn format_citation(reference: &Entry, no_author: bool) -> String {
    let date = format_date(reference.date());

    if no_author {
        date
    } else if let Some(authors) = reference.author() {
        format!("{} {}", authors[0].name, date)
    } else {
        format!("Anonymous {}", date)
    }
}

pub fn format_reference(item: &Entry) -> String {
    let mut result = String::new();
    write!(
        result,
        "[{}] {} ({}): **{}**",
        item.key,
        format_authors(item.author()),
        format_date(item.date()),
        item.title()
            .map(|chunks| chunks.format_verbatim())
            .unwrap_or_else(|| "Untitled".to_string()),
    )
    .unwrap();

    if let Some(chunks) = item.journal() {
        write!(result, ". *{}*", chunks.format_verbatim()).unwrap();
    }

    if let Some(volume) = item.volume() {
        write!(result, " {}", volume).unwrap();

        if let Some(number) = item.number() {
            write!(result, ":{}", number.format_verbatim()).unwrap();
        }
    }

    if let Some(ranges) = item.pages() {
        write!(result, ", {}-{}", &ranges[0].start, &ranges[0].end).unwrap();
    }

    write!(result, ".").unwrap();

    result
}

fn format_authors(authors: Option<Vec<Person>>) -> String {
    let mut result = String::new();
    if let Some(authors) = authors {
        for (idx, author) in authors.iter().enumerate() {
            write!(result, "{}", author.name).unwrap();
            if !author.given_name.is_empty() {
                write!(result, " {}", &author.given_name[..1]).unwrap();
            }
            if idx < authors.len() - 1 {
                write!(result, ", ").unwrap();
            }
        }
    } else {
        write!(result, "Anonymous").unwrap();
    }
    result
}

fn format_date(date: Option<Date>) -> String {
    if let Some(date) = date {
        if let DateValue::At(time) = date.value {
            format!("{}", time.year)
        } else {
            "????".to_owned()
        }
    } else {
        "????".to_owned()
    }
}
