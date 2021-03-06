use crate::format::EntryFormatter;
use biblatex::{ChunksExt, Entry};
use std::fmt::Write;

pub struct ArticleFormatter {}

impl EntryFormatter for ArticleFormatter {
    fn format(&self, result: &mut dyn Write, item: &Entry) {
        write!(
            result,
            "{} ({}): **{}**",
            super::format_authors(item.author()),
            super::format_date(item.date()),
            super::format_title(item.title()),
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
            write!(result, ", {}", super::format_pages(&ranges[..])).unwrap();
        }

        write!(result, ".").unwrap();
    }
}
