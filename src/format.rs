use crate::config::{CitationStyle, Config};
use biblatex::{ChunksExt, Date, DateValue, Entry, Person};
use std::fmt::Write;

pub fn format_citation(
    reference: &Entry,
    index: usize,
    link_prefix: Option<&String>,
    no_author: bool,
    config: &Config,
) -> String {
    let anchor = format_ref_anchor(&reference.key);
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

pub fn format_reference(item: &Entry, index: usize, config: &Config) -> String {
    let mut result = String::new();
    let anchor = format_ref_anchor(&item.key);

    if config.link_refs {
        write!(result, "<a name=\"{}\" id=\"{}\"></a>", anchor, anchor,).unwrap();
    }

    if config.citation_style == CitationStyle::Index {
        write!(result, "[{}] ", index).unwrap();
    }
    if config.render_key {
        write!(result, "[{}] ", item.key).unwrap();
    }

    write!(
        result,
        "{} ({}): **{}**",
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

fn format_ref_anchor(key: &str) -> String {
    format!("cite-ref-{}", key)
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
    use crate::bib::parse_bibliography;
    use crate::config::{CitationStyle, Config};

    const TEST_BIB: &str = r#"
@book{Klabnik2018,
    author = {"Klabnik, Steve and Nichols, Carol"},
    title = {"The Rust Programming Language"},
    year = {"2018"},
    isbn = {"1593278284"},
    publisher = {"No Starch Press"},
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

        let bib = parse_bibliography(TEST_BIB).unwrap();

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
