use crate::format::EntryFormatter;
use biblatex::Entry;
use std::fmt::Write;

pub struct BookFormatter {}

impl EntryFormatter for BookFormatter {
    fn format(&self, result: &mut dyn Write, item: &Entry) {
        write!(
            result,
            "{} ({}): **{}**",
            super::format_authors_opt(item.author().as_ref()),
            super::format_date(item.date()),
            super::format_chunk_opt(item.title(), "Untitled"),
        )
        .unwrap();

        if let Some(chunks) = item.publisher() {
            write!(result, ". *{}*", super::format_chunks(&chunks, ", ")).unwrap();
        }

        if let Some(chunks) = item.address() {
            write!(result, ", {}", super::format_chunk(&chunks)).unwrap();
        }

        write!(result, ".").unwrap();
    }
}
