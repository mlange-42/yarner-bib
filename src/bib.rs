use biblatex::Bibliography;
use std::error::Error;
use std::path::Path;

pub fn load_bibliography<P: AsRef<Path>>(file: P) -> Result<Bibliography, Box<dyn Error>> {
    let content = std::fs::read_to_string(&file).map_err(|err| {
        format!(
            "Can't read bibliography from file {} - {}",
            file.as_ref().display(),
            err.to_string()
        )
    })?;

    Bibliography::parse(&content)
        .ok_or_else(|| format!("No valid bibliography in file {}", file.as_ref().display()).into())
}
