# getlink

A cross-platform command-line tool written in Rust to extract and print all relevant sub-links from a target web page.

It fetches the provided URL, parses the HTML, and prints all the `href` links that belong to the same host and share the root path prefix. The output is a newline-separated list of URLs, sorted alphabetically. **The results are also automatically copied to your system clipboard.**

## Features

- **Blazing Fast**: Uses `reqwest` and `scraper` in Rust for fast synchronous fetching and parsing.
- **Path Filtering**: Intelligently keeps only links that are children of your target URL (e.g., if you crawl `https://docs.rs/tokio/latest/tokio/`, it ignores links mapping back to `https://docs.rs/` or external sites).
- **Cross-Platform**: Works on Windows, macOS, and Linux.

## Installation

### From Pre-built Binaries (GitHub Releases)

You can download pre-compiled binaries for your operating system from the [Releases](#) page of this repository.

### Build From Source

Make sure you have [Rust and Cargo](https://rustup.rs/) installed, then:

```bash
git clone https://github.com/bluetweed/getlink-cli.git
cd getlink-cli
cargo install --path .
```

## Usage

Provide the root page URL as the only argument:

```bash
getlink https://docs.rs/tokio/latest/tokio/
```

### Example output:

```text
https://docs.rs/tokio/latest/tokio/
https://docs.rs/tokio/latest/tokio/all.html
https://docs.rs/tokio/latest/tokio/attr.main.html
https://docs.rs/tokio/latest/tokio/attr.test.html
...
```

## License

MIT License
