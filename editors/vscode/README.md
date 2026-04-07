# Holy Language

VS Code extension for [Holy Lang](https://github.com/gaok1/holy-lang) — a custom scripting language.

## Features

- Syntax highlighting for `.holy` files
- Code snippets
- File icon for `.holy` files (enable via **File Icon Theme → Holy Language Icons**)
- **Run button** (▶) in the editor title bar — requires `holy` in your PATH

## Requirements

Install the interpreter and add it to your PATH:

```sh
git clone https://github.com/gaok1/holy-lang
cd holy-lang
cargo install --path .
```

## Usage

Open any `.holy` file and click the **▶** button in the top-right corner of the editor.
