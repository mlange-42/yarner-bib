use crate::config::{CitationStyle, Config};
use crate::format;
use biblatex::Bibliography;
use linked_hash_map::{Entry, LinkedHashMap};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use yarner_lib::{Document, Node, TextBlock};

const REF_PATTERN: &str = r##"(-)?@([^\[\]\s\.,;"#'()={}%]+)"##;
static REF_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(REF_PATTERN).unwrap());

pub fn insert_references(
    path: &PathBuf,
    document: &mut Document,
    citations: &LinkedHashMap<String, usize>,
    bibliography: &Bibliography,
    config: &Config,
) {
    let mut pattern_found = false;

    for node in document.nodes.iter_mut() {
        if let Node::Text(block) = node {
            for line_idx in 0..block.text.len() {
                if block.text[line_idx].contains(&config.placeholder) {
                    let refs = render_references(&citations, &bibliography, &config);
                    block.text = block
                        .text
                        .iter()
                        .take(line_idx)
                        .chain(refs.iter())
                        .chain(block.text.iter().skip(line_idx + 1))
                        .cloned()
                        .collect();
                    pattern_found = true;
                    break;
                }
            }
        }
    }

    if !pattern_found && !citations.is_empty() {
        eprintln!(
            "  Warning: no placeholder for references found in {}",
            path.display()
        );
    }
}

fn render_references(
    citations: &LinkedHashMap<String, usize>,
    bibliography: &Bibliography,
    config: &Config,
) -> Vec<String> {
    let mut text = vec![];

    let bib: Vec<_> = match config.citation_style {
        CitationStyle::Index => citations
            .iter()
            .filter_map(|(key, idx)| bibliography.get(key).map(|e| (e, idx)))
            .collect(),
        CitationStyle::AuthorYear => {
            let mut bib: Vec<_> = bibliography
                .iter()
                .filter_map(|entry| citations.get(&entry.key).map(|idx| (entry, idx)))
                .collect();
            bib.sort_by_cached_key(|(entry, _idx)| {
                (entry.author(), format::format_date(entry.date()))
            });
            bib
        }
    };

    for (item, idx) in bib.iter() {
        text.push(format::format_reference(
            item,
            &config.citation_style,
            config.render_key,
            *idx + 1,
        ));
        text.push("".to_string());
    }
    text.pop();

    text
}

pub fn render_citations(
    document: &mut Document,
    bibliography: &Bibliography,
    config: &Config,
) -> LinkedHashMap<String, usize> {
    let mut citations = LinkedHashMap::new();

    for mut node in document.nodes.iter_mut() {
        if let Node::Text(block) = &mut node {
            render_citations_block(
                block,
                bibliography,
                &config.citation_style,
                None,
                &mut citations,
            );
        }
    }

    citations
}

pub fn render_citations_all(
    documents: &mut HashMap<PathBuf, Document>,
    bibliography: &Bibliography,
    config: &Config,
    refs_file: &PathBuf,
) -> LinkedHashMap<String, usize> {
    let mut citations = LinkedHashMap::new();

    for (path, doc) in documents.iter_mut() {
        let rel_link = if path == refs_file {
            None
        } else {
            Some(relative_link(refs_file, path))
        };
        for mut node in doc.nodes.iter_mut() {
            if let Node::Text(block) = &mut node {
                render_citations_block(
                    block,
                    bibliography,
                    &config.citation_style,
                    rel_link.as_ref(),
                    &mut citations,
                );
            }
        }
    }

    citations
}

fn relative_link<P, B>(abs_link: P, root: B) -> String
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    pathdiff::diff_paths(&abs_link, root.as_ref().parent().unwrap())
        .and_then(|p| p.as_path().to_str().map(|s| s.replace('\\', "/")))
        .unwrap_or_else(|| "invalid path".to_owned())
}

fn render_citations_block(
    block: &mut TextBlock,
    bibliography: &Bibliography,
    style: &CitationStyle,
    link_prefix: Option<&String>,
    citations: &mut LinkedHashMap<String, usize>,
) {
    for line in block.text.iter_mut() {
        if REF_REGEX.is_match(&line) {
            let ln = REF_REGEX.replace_all(line, |caps: &Captures| {
                let no_author = caps.get(1).is_some();
                let key = &caps[2];
                if let Some(reference) = bibliography.get(key) {
                    let index = citations.len();
                    let ref_index = match citations.entry(key.to_owned()) {
                        Entry::Occupied(entry) => *entry.get(),
                        Entry::Vacant(entry) => *entry.insert(index),
                    };
                    format::format_citation(reference, ref_index + 1, style, link_prefix, no_author)
                } else {
                    let original = caps.get(0).unwrap().as_str().to_owned();
                    eprintln!(
                        "  Warning: citation key '{}' not found in bibliography.",
                        original
                    );
                    original
                }
            });

            *line = ln.to_string();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bib::parse_bibliography;
    use crate::config::CitationStyle;
    use linked_hash_map::LinkedHashMap;
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
    fn render_citations_block() {
        let bib = parse_bibliography(TEST_BIB).unwrap();
        let mut citations = LinkedHashMap::new();

        let mut block = TextBlock {
            text: vec!["A test citation: @Klabnik2018.".to_string()],
        };

        super::render_citations_block(
            &mut block,
            &bib,
            &CitationStyle::AuthorYear,
            None,
            &mut citations,
        );

        assert_eq!(citations.len(), 1);
        assert_eq!(
            &block.text[0],
            "A test citation: [Klabnik & Nichols 2018](#cite-ref-Klabnik2018)."
        )
    }

    #[test]
    fn render_citations_block_no_author() {
        let bib = parse_bibliography(TEST_BIB).unwrap();
        let mut citations = LinkedHashMap::new();

        let mut block = TextBlock {
            text: vec!["A test citation: -@Klabnik2018.".to_string()],
        };

        super::render_citations_block(
            &mut block,
            &bib,
            &CitationStyle::AuthorYear,
            None,
            &mut citations,
        );

        assert_eq!(citations.len(), 1);
        assert_eq!(
            &block.text[0],
            "A test citation: [2018](#cite-ref-Klabnik2018)."
        )
    }
}
