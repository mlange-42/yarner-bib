use biblatex::{Bibliography, DateValue, Entry};
use linked_hash_set::LinkedHashSet;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::PathBuf;
use yarner_lib::{Document, Node, TextBlock};

const REF_PATTERN: &str = r##"(-)?@([^\[\]\s\."#'(),={}%]+)"##;
static REF_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(REF_PATTERN).unwrap());

pub fn render_citations(
    documents: &mut HashMap<PathBuf, Document>,
    bibliography: &Bibliography,
) -> LinkedHashSet<String> {
    let mut citations = LinkedHashSet::new();

    for (_path, doc) in documents.iter_mut() {
        for mut node in doc.nodes.iter_mut() {
            if let Node::Text(block) = &mut node {
                render_citations_block(block, bibliography, &mut citations);
            }
        }
    }

    citations
}

fn render_citations_block(
    block: &mut TextBlock,
    bibliography: &Bibliography,
    citations: &mut LinkedHashSet<String>,
) {
    for line in block.text.iter_mut() {
        if REF_REGEX.is_match(&line) {
            let ln = REF_REGEX.replace_all(line, |caps: &Captures| {
                let no_author = caps.get(1).is_some();
                let key = &caps[2];
                if let Some(reference) = bibliography.get(key) {
                    citations.insert(key.to_owned());
                    render_citation(reference, no_author)
                } else {
                    caps.get(0).unwrap().as_str().to_owned()
                }
            });

            *line = ln.to_string();
        }
    }
}

fn render_citation(reference: &Entry, no_author: bool) -> String {
    let date = if let Some(date) = reference.date() {
        if let DateValue::At(time) = date.value {
            format!("{}", time.year)
        } else {
            "????".to_owned()
        }
    } else {
        "????".to_owned()
    };

    if no_author {
        date
    } else if let Some(authors) = reference.author() {
        format!("{} {}", authors[0].name, date)
    } else {
        format!("Anonymous {}", date)
    }
}

#[cfg(test)]
mod test {
    use crate::bib::parse_bibliography;
    use linked_hash_set::LinkedHashSet;
    use yarner_lib::TextBlock;

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
    fn render_citation() {
        let bib = parse_bibliography(TEST_BIB).unwrap();

        assert_eq!(
            super::render_citation(bib.get("Klabnik2018").unwrap(), false),
            "Klabnik 2018"
        );

        assert_eq!(
            super::render_citation(bib.get("Klabnik2018").unwrap(), true),
            "2018"
        );
    }

    #[test]
    fn render_citations_block() {
        let bib = parse_bibliography(TEST_BIB).unwrap();
        let mut citations = LinkedHashSet::new();

        let mut block = TextBlock {
            text: vec!["A test citation: @Klabnik2018.".to_string()],
        };

        super::render_citations_block(&mut block, &bib, &mut citations);

        assert_eq!(citations.len(), 1);
        assert_eq!(&block.text[0], "A test citation: Klabnik 2018.")
    }

    #[test]
    fn render_citations_block_no_author() {
        let bib = parse_bibliography(TEST_BIB).unwrap();
        let mut citations = LinkedHashSet::new();

        let mut block = TextBlock {
            text: vec!["A test citation: -@Klabnik2018.".to_string()],
        };

        super::render_citations_block(&mut block, &bib, &mut citations);

        assert_eq!(citations.len(), 1);
        assert_eq!(&block.text[0], "A test citation: 2018.")
    }
}
