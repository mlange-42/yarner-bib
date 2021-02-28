use biblatex::Bibliography;
use std::error::Error;
use std::path::Path;
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
    let data = yarner_lib::parse_input()?;
    let config = &data.context.config;

    check_version(&data.context);

    let bib_file = config
        .get("bibliography")
        .and_then(|s| s.as_str())
        .unwrap_or("bibliography.bib");

    let _bibliography = load_bibliography(bib_file)?;

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

fn load_bibliography<P: AsRef<Path>>(file: P) -> Result<Bibliography, Box<dyn Error>> {
    Bibliography::parse(&std::fs::read_to_string(&file).map_err(|err| {
        format!(
            "Can't read bibliography from file {} - {}",
            file.as_ref().display(),
            err.to_string()
        )
    })?)
    .ok_or_else(|| format!("No valid bibliography in file {}", file.as_ref().display()).into())
}
