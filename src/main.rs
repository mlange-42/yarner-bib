mod bib;
mod config;
mod format;
mod render;

use crate::config::Config;
use std::convert::TryFrom;
use std::error::Error;
use std::path::PathBuf;
use yarner_lib::Context;

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            1
        }
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut data = yarner_lib::parse_input()?;
    let config = Config::try_from(&data.context.config)?;

    check_version(&data.context);

    let bibliography = bib::load_bibliography(&config.bib_file)?;

    if let Some(refs_file) = &config.refs_file {
        let path = PathBuf::from(&refs_file);
        if !data.documents.contains_key(&path) {
            return Err(format!(
                "Reference output file {} not in the list of documents. Include it with @[...](...)",
                path.display()
            )
            .into());
        }

        let citations =
            render::render_citations_all(&mut data.documents, &bibliography, &config, &path);

        render::insert_references(
            &path,
            data.documents.get_mut(&path).unwrap(),
            &citations,
            &bibliography,
            &config,
        );
    } else {
        for (path, mut doc) in data.documents.iter_mut() {
            let citations = render::render_citations(&mut doc, &bibliography, &config);
            render::insert_references(path, &mut doc, &citations, &bibliography, &config);
        }
    }

    yarner_lib::write_output(&data)?;
    Ok(())
}

fn check_version(context: &Context) {
    if context.yarner_version != yarner_lib::YARNER_VERSION {
        eprintln!(
            "  Warning: The {} plugin was built against version {} of Yarner, \
                    but we're being called from version {}",
            context.name,
            yarner_lib::YARNER_VERSION,
            context.yarner_version
        )
    }
}
