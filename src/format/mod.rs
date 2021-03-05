mod article;

use crate::config::{CitationStyle, Config};
use biblatex::{Chunk, ChunksExt, Date, DateValue, Entry, EntryType, Person};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Range;

static FORMATTERS: Lazy<HashMap<String, Box<dyn EntryFormatter>>> = Lazy::new(|| {
    let mut map: HashMap<String, Box<dyn EntryFormatter>> = HashMap::new();
    map.insert(
        EntryType::Article.to_string(),
        Box::new(article::ArticleFormatter {}),
    );

    map
});
static DEFAULT_FORMATTER: Lazy<Box<dyn EntryFormatter>> =
    Lazy::new(|| Box::new(article::ArticleFormatter {}));

trait EntryFormatter: Send + Sync {
    fn format(&self, item: &Entry, index: usize, config: &Config) -> String;
}

pub fn format_reference(item: &Entry, index: usize, config: &Config) -> String {
    let formatter = FORMATTERS
        .get(&item.entry_type.to_string())
        .unwrap_or(&DEFAULT_FORMATTER);
    formatter.format(item, index, config)
}

pub fn format_citation(
    reference: &Entry,
    index: usize,
    link_prefix: Option<&String>,
    no_author: bool,
    config: &Config,
) -> String {
    let anchor = key_to_anchor(&reference.key);
    let prefix = link_prefix.cloned().unwrap_or_default();

    if config.link_refs {
        match &config.citation_style {
            CitationStyle::Index => {
                format!("[{}]({}#{})", index, prefix, anchor)
            }
            CitationStyle::AuthorYear => {
                let date = format_date(reference.date());
                if no_author {
                    format!("[{}]({}#{})", date, prefix, anchor)
                } else {
                    format!(
                        "[{} {}]({}#{})",
                        format_authors_citation(reference.author()),
                        date,
                        prefix,
                        anchor
                    )
                }
            }
        }
    } else {
        match &config.citation_style {
            CitationStyle::Index => {
                format!("{}", index)
            }
            CitationStyle::AuthorYear => {
                let date = format_date(reference.date());
                if no_author {
                    date
                } else {
                    format!("{} {}", format_authors_citation(reference.author()), date,)
                }
            }
        }
    }
}

fn format_anchor(key: &str) -> String {
    let anchor = key_to_anchor(key);
    format!("<a name=\"{}\" id=\"{}\"></a>", anchor, anchor,)
}

fn key_to_anchor(key: &str) -> String {
    format!("cite-ref-{}", key)
}

fn format_pages(ranges: &[Range<u32>]) -> String {
    if ranges.is_empty() {
        "???".to_string()
    } else {
        format!("{}-{}", &ranges[0].start, &ranges[0].end)
    }
}

fn format_authors(authors: Option<Vec<Person>>) -> String {
    let mut result = String::new();
    if let Some(authors) = authors {
        for (idx, author) in authors.iter().enumerate() {
            write!(result, "{}", author.name).unwrap();
            if !author.given_name.is_empty() {
                write!(result, " ").unwrap();
                for part in author.given_name.split(' ') {
                    write!(result, "{}", &part[..1]).unwrap();
                }
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

fn format_title(title: Option<&[Chunk]>) -> String {
    title
        .map(|chunks| chunks.format_verbatim())
        .unwrap_or_else(|| "Untitled".to_string())
}

fn format_authors_citation(authors: Option<Vec<Person>>) -> String {
    let mut result = String::new();
    if let Some(authors) = authors {
        match authors.len() {
            1 => write!(result, "{}", authors[0].name).unwrap(),
            2 => write!(result, "{} & {}", authors[0].name, authors[1].name).unwrap(),
            _ => write!(result, "{} et al.", authors[0].name).unwrap(),
        }
    } else {
        write!(result, "Anonymous").unwrap();
    }
    result
}

pub fn format_date(date: Option<Date>) -> String {
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

#[cfg(test)]
mod test {
    use crate::config::{CitationStyle, Config};
    use biblatex::Bibliography;

    const TEST_BIB: &str = r#"
@book{Klabnik2018,
    author = {Klabnik, Steve and Nichols, Carol},
    title = {The Rust Programming Language},
    year = {2018},
    isbn = {1593278284},
    publisher = {No Starch Press},
}
"#;

    #[test]
    fn format_citation() {
        let config = Config {
            bib_file: "".to_string(),
            citation_style: CitationStyle::AuthorYear,
            refs_file: None,
            placeholder: "[[_REFS_]]".to_string(),
            render_key: true,
            link_refs: true,
        };

        let bib = Bibliography::parse(TEST_BIB).unwrap();

        assert_eq!(
            super::format_citation(bib.get("Klabnik2018").unwrap(), 1, None, false, &config),
            "[Klabnik & Nichols 2018](#cite-ref-Klabnik2018)"
        );

        assert_eq!(
            super::format_citation(bib.get("Klabnik2018").unwrap(), 1, None, true, &config),
            "[2018](#cite-ref-Klabnik2018)"
        );
    }
}
