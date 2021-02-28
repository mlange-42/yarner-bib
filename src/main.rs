mod bib;
mod config;
mod render;

use crate::config::Config;
use std::convert::TryFrom;
use std::error::Error;
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

    let bibliography = bib::load_bibliography(config.bib_file)?;

    let _citations = render::render_citations(&mut data.documents, &bibliography);

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
