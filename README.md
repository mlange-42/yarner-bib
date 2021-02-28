# yarner-bib

A [Yarner](https://github.com/mlange-42/yarner) plugin for citations using a BibTeX bibliography.

## Installation

**Binaries**

1. Download the [latest binaries](https://github.com/mlange-42/yarner-bib/releases) for your platform
2. Unzip somewhere
3. Add the parent directory of the executable to your `PATH` environmental variable

**Using `cargo`**

```
> cargo install --git https://github.com/mlange-42/yarner-bib.git --branch main
```

## Usage

Add a section `plugin.bib` to your `Yarner.toml`:

```toml
[plugin.bib]
...
```

The plugin allows for different options, which are all optional:

```toml
[plugin.bib]
bibliography = "bibliography.bib"
style = "author-year"
refs-file = "References.md"
placeholder = "[[_REFS_]]"
render-key = true
```

Cite using the BibTeX citation key, prefixed with `@`:

```markdown
For details, see @Doe2020.
```

To generate the reference list, place the placeholder in each file, or in the file given under `refs-file`:

```markdown
## References

[[_REFS_]]
```
