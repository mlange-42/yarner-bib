# yarner-bib

[![Tests](https://github.com/mlange-42/yarner-bib/actions/workflows/tests.yml/badge.svg)](https://github.com/mlange-42/yarner-bib/actions/workflows/tests.yml)

A [Yarner](https://github.com/mlange-42/yarner) plugin for citations using a BibTeX bibliography.

Example:

Here is a Markdown source with citations and a placeholder for the references:

```markdown
## Yarner-bib example

Yarner is a command line tool for Literate Programming (@Knuth1984).
Another famous Literate Programming environment is RMarkdown (@Baumer2015).

## References

[[_REFS_]]
```

After processing with Yarner and yarner-bib, it produces this output:

<table><tr><td>

## Yarner-bib example

Yarner is a command line tool for Literate Programming ([Knuth 1984](#cite-ref-Knuth1984)). Another famous Literate Programming environment is RMarkdown ([Baumer & Udwin 2015](#cite-ref-Baumer2015)).

## References

<a name="cite-ref-Baumer2015" id="cite-ref-Baumer2015"></a>Baumer B, Udwin D (2015): **R Markdown**. *WIREs Computational Statistics* 7:3, 167-177.

<a name="cite-ref-Knuth1984" id="cite-ref-Knuth1984"></a>Knuth DE (1984): **Literate Programming**. *The Computer Journal* 27:2, 97-111.
</td></tr></table>

## Installation

**Binaries**

1. Download the [latest binaries](https://github.com/mlange-42/yarner-bib/releases) for your platform
2. Unzip somewhere
3. Add the parent directory of the executable to your `PATH` environmental variable

**Using `cargo`**

```
> cargo install yarner-bib
```

## Usage

Add a section `plugin.bib` to your `Yarner.toml`:

```toml
[plugin.bib]
...
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

## Options

The plugin allows for different options, which are all optional:

```toml
[plugin.bib]
bibliography = "bibliography.bib"
style = "author-year"
refs-file = "References.md"
placeholder = "[[_REFS_]]"
link-refs = true
render-key = true
```

| Option         | Details                                                                 | Default              |
|----------------|-------------------------------------------------------------------------|----------------------|
| `bibliography` | The BibTeX file to use.                                                 | `bibliography.bib`   |
| `style`        | The citation style. Either `author-year` or `numbered`.                 | `author-year`        |
| `refs-file`    | The Markdown file for references. References in each file if not given. | none                 |
| `placeholder`  | The placeholder to replace by the list of references                    | `[[_REFS_]]`         |
| `link-refs`    | Add links from citations to references.                                 | `true`               |
| `render-key`   | Render the citation key in front of each reference.                     | `true`               |
