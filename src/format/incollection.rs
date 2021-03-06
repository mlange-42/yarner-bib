use crate::format::EntryFormatter;
use biblatex::Entry;
use std::fmt::Write;

pub struct InCollectionFormatter {}

impl EntryFormatter for InCollectionFormatter {
    fn format(&self, result: &mut dyn Write, item: &Entry) {
        write!(
            result,
            "{} ({}): **{}**",
            super::format_authors_opt(item.author().as_ref()),
            super::format_date(item.date()),
            super::format_chunk_opt(item.title(), "Untitled"),
        )
        .unwrap();

        write!(
            result,
            ". In: {} (eds.): {}",
            super::format_authors_opt(item.editors().get(0).map(|e| &e.0)),
            super::format_chunk_opt(item.book_title(), "Untitled"),
        )
        .unwrap();

        if let Some(ranges) = item.pages() {
            write!(result, ", pp. {}", super::format_pages(&ranges[..])).unwrap();
        }

        if let Some(chunks) = item.publisher() {
            write!(result, ". *{}*", super::format_chunks(&chunks, ", ")).unwrap();
        }

        if let Some(chunks) = item.address() {
            write!(result, ", {}", super::format_chunk(&chunks)).unwrap();
        }

        write!(result, ".").unwrap();
    }
}
