use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

#[derive(PartialEq)]
pub enum CitationStyle {
    Index,
    AuthorYear,
}

impl FromStr for CitationStyle {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "numbered" => Ok(CitationStyle::Index),
            "author-year" => Ok(CitationStyle::AuthorYear),
            other => Err(format!(
                "Unknown citation style '{}'. Use 'numbered' or 'author-year'",
                other
            )
            .into()),
        }
    }
}

pub struct Config {
    pub bib_file: String,
    pub citation_style: CitationStyle,
    pub refs_file: Option<String>,
    pub placeholder: String,
    pub render_key: bool,
}

impl TryFrom<&toml::Value> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: &toml::Value) -> Result<Self, Self::Error> {
        Ok(Self {
            bib_file: value
                .get("bibliography")
                .and_then(|s| s.as_str())
                .unwrap_or("bibliography.bib")
                .to_owned(),
            citation_style: value
                .get("style")
                .and_then(|s| s.as_str())
                .map(|s| CitationStyle::from_str(s))
                .unwrap_or(Ok(CitationStyle::AuthorYear))?,
            refs_file: value
                .get("refs-file")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string()),
            placeholder: value
                .get("placeholder")
                .and_then(|s| s.as_str())
                .unwrap_or("[[_REFS_]]")
                .to_owned(),
            render_key: value
                .get("render-key")
                .and_then(|s| s.as_bool())
                .unwrap_or(true),
        })
    }
}
